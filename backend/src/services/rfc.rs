use crate::config::Config;
use crate::db::DbPool;
use crate::error::{AppError, Result};
use crate::models::{Rfc, RfcListItem, SearchRfcQuery, SearchRfcResponse, Translation};

/// 获取RFC列表（不含原文）
pub async fn list_rfcs(db: &DbPool) -> Result<Vec<RfcListItem>> {
    let rfcs = sqlx::query_as::<_, RfcListItem>(
        r#"SELECT id, rfc_number, title, status, abstract, publish_date, created_at 
           FROM rfcs 
           ORDER BY rfc_number ASC 
           LIMIT 100"#
    )
    .fetch_all(db)
    .await?;

    Ok(rfcs)
}

/// 搜索RFC
pub async fn search_rfcs(db: &DbPool, query: SearchRfcQuery) -> Result<SearchRfcResponse> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100).max(1);
    let offset = (page - 1) * per_page;

    let (rfcs, total) = if let Some(ref keyword) = query.q {
        // 关键词搜索
        let search_pattern = format!("%{}%", keyword);
        
        let rfcs = sqlx::query_as::<_, RfcListItem>(
            r#"SELECT id, rfc_number, title, status, abstract, publish_date, created_at 
               FROM rfcs 
               WHERE title ILIKE $1 OR abstract ILIKE $1 OR rfc_number::text LIKE $1
               ORDER BY rfc_number ASC 
               LIMIT $2 OFFSET $3"#
        )
        .bind(&search_pattern)
        .bind(per_page)
        .bind(offset)
        .fetch_all(db)
        .await?;

        let total: (i64,) = sqlx::query_as(
            r#"SELECT COUNT(*) FROM rfcs 
               WHERE title ILIKE $1 OR abstract ILIKE $1 OR rfc_number::text LIKE $1"#
        )
        .bind(&search_pattern)
        .fetch_one(db)
        .await?;

        (rfcs, total.0)
    } else {
        // 默认列表
        let rfcs = sqlx::query_as::<_, RfcListItem>(
            r#"SELECT id, rfc_number, title, status, abstract, publish_date, created_at 
               FROM rfcs 
               ORDER BY rfc_number ASC 
               LIMIT $1 OFFSET $2"#
        )
        .bind(per_page)
        .bind(offset)
        .fetch_all(db)
        .await?;

        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM rfcs")
            .fetch_one(db)
            .await?;

        (rfcs, total.0)
    };

    let total_pages = ((total as f64) / (per_page as f64)).ceil() as i32;

    Ok(SearchRfcResponse {
        rfcs,
        total,
        page,
        per_page,
        total_pages,
    })
}

/// 根据RFC编号获取详情
pub async fn get_rfc_by_number(db: &DbPool, rfc_number: i32) -> Result<Rfc> {
    let rfc = sqlx::query_as::<_, Rfc>("SELECT * FROM rfcs WHERE rfc_number = $1")
        .bind(rfc_number)
        .fetch_optional(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("RFC {} 不存在", rfc_number)))?;

    Ok(rfc)
}



/// 获取RFC的所有翻译
pub async fn get_rfc_translations(db: &DbPool, rfc_number: i32) -> Result<Vec<Translation>> {
    let rfc = get_rfc_by_number(db, rfc_number).await?;
    
    let translations = sqlx::query_as::<_, Translation>(
        r#"SELECT * FROM translations 
           WHERE rfc_id = $1 
           ORDER BY section_id ASC"#
    )
    .bind(rfc.id)
    .fetch_all(db)
    .await?;

    Ok(translations)
}

/// 从IETF抓取RFC文档
pub async fn fetch_rfc(db: &DbPool, config: &Config, rfc_number: i32) -> Result<Rfc> {
    // 检查RFC是否已存在
    let existing_rfc = sqlx::query_as::<_, Rfc>("SELECT * FROM rfcs WHERE rfc_number = $1")
        .bind(rfc_number)
        .fetch_optional(db)
        .await?;

    if let Some(rfc) = existing_rfc {
        return Ok(rfc);
    }

    // 从IETF抓取RFC文本
    let url = format!("{}rfc{}.txt", config.rfc_base_url, rfc_number);
    tracing::info!("正在抓取 RFC {}: {}", rfc_number, url);

    let response = reqwest::get(&url)
        .await
        .map_err(|e| AppError::ExternalApiError(format!("抓取RFC失败: {}", e)))?;

    if !response.status().is_success() {
        return Err(AppError::NotFound(format!("RFC {} 不存在", rfc_number)));
    }

    let original_text = response
        .text()
        .await
        .map_err(|e| AppError::ExternalApiError(format!("读取RFC内容失败: {}", e)))?;

    // 提取标题和摘要
    let (title, abstract_text) = extract_metadata(&original_text);

    // 保存到数据库
    let rfc = sqlx::query_as::<_, Rfc>(
        r#"INSERT INTO rfcs (rfc_number, title, original_text, abstract, status) 
           VALUES ($1, $2, $3, $4, $5) 
           RETURNING *"#
    )
    .bind(rfc_number)
    .bind(title)
    .bind(&original_text)
    .bind(abstract_text)
    .bind("fetching")
    .fetch_one(db)
    .await?;

    tracing::info!("RFC {} 抓取成功", rfc_number);

    Ok(rfc)
}

/// 更新RFC状态
pub async fn update_rfc_status(db: &DbPool, rfc_id: i32, status: String) -> Result<()> {
    sqlx::query("UPDATE rfcs SET status = $1 WHERE id = $2")
        .bind(status)
        .bind(rfc_id)
        .execute(db)
        .await?;

    Ok(())
}

/// 提取RFC元数据（标题和摘要）
fn extract_metadata(text: &str) -> (String, Option<String>) {
    let mut title = "未知标题".to_string();
    let mut abstract_text: Option<String> = None;
    let mut in_abstract = false;
    let mut abstract_lines: Vec<String> = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();
        
        // 查找标题（通常在前50行）
        if title == "未知标题" && !trimmed.is_empty() 
            && !trimmed.starts_with("RFC")
            && !trimmed.starts_with("Internet")
            && !trimmed.starts_with("Request")
            && !trimmed.starts_with("Network Working Group")
            && !trimmed.starts_with("Category:")
            && !trimmed.starts_with("ISSN:")
            && trimmed.len() > 10 
            && trimmed.len() < 200 
        {
            title = trimmed.to_string();
        }

        // 查找Abstract部分
        if trimmed.eq_ignore_ascii_case("abstract") || trimmed.eq_ignore_ascii_case("abstract:") {
            in_abstract = true;
            continue;
        }

        // 收集摘要内容
        if in_abstract {
            // 遇到下一个主要章节时停止
            if trimmed.starts_with("1.") || trimmed.eq_ignore_ascii_case("introduction")
                || trimmed.eq_ignore_ascii_case("table of contents") 
                || trimmed.eq_ignore_ascii_case("status of this memo")
            {
                break;
            }
            if !trimmed.is_empty() {
                abstract_lines.push(trimmed.to_string());
            }
        }
    }

    if !abstract_lines.is_empty() {
        abstract_text = Some(abstract_lines.join(" "));
    }

    (title, abstract_text)
}
