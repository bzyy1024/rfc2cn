use uuid::Uuid;

use crate::config::Config;
use crate::db::DbPool;
use crate::error::{AppError, Result};
use crate::models::TranslationTask;

use super::rfc::get_rfc_by_number;

pub async fn create_translation_task(
    db: &DbPool,
    _config: &Config,
    rfc_number: i32,
) -> Result<TranslationTask> {
    // 确保RFC存在
    let rfc = get_rfc_by_number(db, rfc_number).await?;

    // 检查是否已有进行中的任务
    let existing_task = sqlx::query_as::<_, TranslationTask>(
        "SELECT * FROM translation_tasks 
         WHERE rfc_id = $1 AND status IN ('pending', 'in_progress')"
    )
    .bind(rfc.id)
    .fetch_optional(db)
    .await?;

    if let Some(task) = existing_task {
        return Ok(task);
    }

    // 创建新的翻译任务
    let task = sqlx::query_as::<_, TranslationTask>(
        "INSERT INTO translation_tasks (id, rfc_id, status, progress) 
         VALUES ($1, $2, $3, $4) 
         RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(rfc.id)
    .bind("pending")
    .bind(0)
    .fetch_one(db)
    .await?;

    tracing::info!("为 RFC {} 创建翻译任务: {}", rfc_number, task.id);

    // TODO: 在实际实现中，这里应该启动后台任务进行翻译
    // tokio::spawn(async move { ... });

    Ok(task)
}

pub async fn get_translation_task(db: &DbPool, task_id: Uuid) -> Result<TranslationTask> {
    let task = sqlx::query_as::<_, TranslationTask>(
        "SELECT * FROM translation_tasks WHERE id = $1"
    )
    .bind(task_id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("翻译任务 {} 不存在", task_id)))?;

    Ok(task)
}
