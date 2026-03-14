//! RFC CLI 工具
//! 
//! 用于本地管理RFC文档：添加、翻译等
//! 
//! 用法:
//!   rfc-cli add <rfc_number>                        # 添加新的RFC
//!   rfc-cli translate <rfc_number> [--force]        # 翻译指定RFC
//!   rfc-cli list                                     # 列出所有RFC
//!   rfc-cli status <rfc_number>                      # 查看RFC状态
//!   rfc-cli parse <rfc_number>                       # 解析RFC结构

use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::{Arc, Mutex};
use std::time::Duration;

// 引入主程序的模块
#[path = "../config.rs"]
mod config;
#[path = "../db.rs"]
mod db;
#[path = "../error.rs"]
mod error;
#[path = "../models/mod.rs"]
mod models;
#[path = "../services/mod.rs"]
mod services;

use config::Config;
use error::Result;

#[derive(Parser)]
#[command(name = "rfc-cli")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 添加新的RFC
    Add {
        /// RFC编号
        rfc_number: i32,
        /// 是否自动翻译
        #[arg(short, long, default_value_t = true)]
        translate: bool,
    },
    /// 翻译RFC
    Translate {
        rfc_number: i32,
        #[arg(short, long)]
        force: bool,
    },
    /// 列出RFC
    List {
        /// 筛选状态
        #[arg(short, long)]
        status: Option<String>,
    },
    /// 列出所有标签
    Tags,
    /// 为RFC添加标签（已禁用）
    Tag {
        /// RFC编号
        rfc_number: i32,
        /// 标签列表（逗号分隔）
        tags: String,
    },
    /// 查看RFC状态
    Status {
        /// RFC编号
        rfc_number: i32,
    },
    /// 解析RFC文档结构
    Parse {
        /// RFC编号
        rfc_number: i32,
    },
    /// 检查Ollama服务状态
    CheckOllama,
    /// 自动同步RFC（从指定范围或最新的RFC）
    Sync {
        /// 起始RFC编号
        #[arg(short, long, default_value = "1")]
        start: i32,
        /// 结束RFC编号（不指定则同步到最新）
        #[arg(short, long)]
        end: Option<i32>,
        /// 跳过翻译，只添加RFC
        #[arg(long)]
        skip_translate: bool,
        /// 并发数量
        #[arg(short, long, default_value = "1")]
        concurrent: usize,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rfc_cli=info".into()),
        )
        .init();

    // 加载配置
    let config = Config::from_env()?;
    
    // 连接数据库
    let db = db::create_pool(&config.database_url).await?;

    // 解析命令行参数
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { rfc_number, translate } => {
            cmd_add(&db, &config, rfc_number, translate).await?;
        }
        Commands::Translate { rfc_number, force } => {
            cmd_translate(&db, &config, rfc_number, force).await?;
        }
        Commands::List { status } => {
            cmd_list(&db, status).await?;
        }
        Commands::Tags => {
            cmd_tags(&db).await?;
        }
        Commands::Tag { rfc_number, tags } => {
            cmd_tag(&db, rfc_number, &tags).await?;
        }
        Commands::Status { rfc_number } => {
            cmd_status(&db, rfc_number).await?;
        }
        Commands::Parse { rfc_number } => {
            cmd_parse(&db, rfc_number).await?;
        }
        Commands::CheckOllama => {
            cmd_check_ollama(&config).await?;
        }
        Commands::Sync { start, end, skip_translate, concurrent } => {
            cmd_sync(&db, &config, start, end, skip_translate, concurrent).await?;
        }
    }

    Ok(())
}

/// 添加新的RFC
async fn cmd_add(
    db: &db::DbPool,
    config: &Config,
    rfc_number: i32,
    auto_translate: bool,
) -> Result<()> {
    println!("🔍 正在获取 RFC {}...", rfc_number);

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("正在从IETF下载...");

    // 抓取RFC
    let rfc = services::rfc::fetch_rfc(db, config, rfc_number).await?;
    pb.finish_with_message(format!("✅ RFC {} 下载完成: {}", rfc_number, rfc.title));

    // 解析RFC结构
    println!("📝 正在解析RFC结构...");
    parse_and_save_sections(db, &rfc).await?;
    println!("✅ RFC结构解析完成");

    // 自动翻译
    if auto_translate {
        cmd_translate(db, config, rfc_number, false).await?;
    }

    Ok(())
}

/// 翻译RFC
async fn cmd_translate(
    db: &db::DbPool,
    config: &Config,
    rfc_number: i32,
    force: bool,
) -> Result<()> {
    println!("🌐 开始翻译 RFC {}...", rfc_number);

    // 获取RFC
    let rfc = services::rfc::get_rfc_by_number(db, rfc_number).await?;

    // 获取待翻译的段落
    let translations = sqlx::query_as::<_, models::Translation>(
        if force {
            "SELECT * FROM translations WHERE rfc_id = $1 ORDER BY section_id"
        } else {
            "SELECT * FROM translations WHERE rfc_id = $1 AND translated_text IS NULL ORDER BY section_id"
        }
    )
    .bind(rfc.id)
    .fetch_all(db)
    .await?;

    if translations.is_empty() {
        println!("✅ 没有需要翻译的内容");
        return Ok(());
    }

    println!("📋 共有 {} 个段落需要翻译", translations.len());

    // 检查AI配置
    match &config.ai_provider {
        Some(provider) if provider == "ollama" => {
            if !services::ai::check_ollama_health(config).await? {
                return Err(error::AppError::ExternalApiError(
                    "Ollama服务不可用，请确保Ollama正在运行".to_string()
                ));
            }
            let default_model = "未配置".to_string();
            let model = config.ollama_model.as_ref().unwrap_or(&default_model);
            println!("✅ Ollama服务连接正常，使用模型: {}", model);
        },
        Some(provider) if provider == "openai" => {
            if config.openai_api_key.is_none() {
                return Err(error::AppError::InternalError(
                    "OpenAI API Key 未配置".to_string()
                ));
            }
            println!("✅ 使用 OpenAI API 进行翻译");
        },
        None => {
            return Err(error::AppError::InternalError(
                "AI翻译功能未启用。请设置 AI_PROVIDER 环境变量（ollama 或 openai）".to_string()
            ));
        },
        _ => {
            return Err(error::AppError::InternalError(
                format!("不支持的 AI 提供商: {:?}", config.ai_provider)
            ));
        }
    }

    // 更新RFC状态
    services::rfc::update_rfc_status(db, rfc.id, "translating".to_string()).await?;

    // 创建进度条
    let pb = ProgressBar::new(translations.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    let mut success_count = 0;
    let mut error_count = 0;

    for translation in translations {
        pb.set_message(format!("翻译段落: {}", translation.section_id));

        match services::ai::translate_text(config, &translation.original_text).await {
            Ok(translated) => {
                // 保存翻译结果
                sqlx::query(
                    "UPDATE translations SET translated_text = $1 WHERE id = $2"
                )
                .bind(&translated)
                .bind(translation.id)
                .execute(db)
                .await?;
                success_count += 1;
            }
            Err(e) => {
                tracing::error!("翻译段落 {} 失败: {}", translation.section_id, e);
                error_count += 1;
            }
        }

        pb.inc(1);
    }

    pb.finish_with_message("翻译完成");

    // 更新RFC状态
    let new_status = if error_count == 0 {
        "completed"
    } else {
        "reviewing"
    };
    services::rfc::update_rfc_status(db, rfc.id, new_status.to_string()).await?;

    println!("\n📊 翻译结果:");
    println!("   ✅ 成功: {}", success_count);
    println!("   ❌ 失败: {}", error_count);

    Ok(())
}

/// 列出所有RFC
async fn cmd_list(db: &db::DbPool, status_filter: Option<String>) -> Result<()> {
    let rfcs = if let Some(status) = status_filter {
        sqlx::query_as::<_, models::RfcListItem>(
            "SELECT id, rfc_number, title, status, abstract, publish_date, created_at 
             FROM rfcs WHERE status::text = $1 ORDER BY rfc_number DESC"
        )
        .bind(status)
        .fetch_all(db)
        .await?
    } else {
        services::rfc::list_rfcs(db).await?
    };

    if rfcs.is_empty() {
        println!("📭 没有找到RFC文档");
        return Ok(());
    }

    println!("\n📚 RFC列表 (共 {} 个):\n", rfcs.len());
    println!("{:<8} {:<60} {:<12}", "编号", "标题", "状态");
    println!("{}", "-".repeat(82));

    for rfc in rfcs {
        let title = if rfc.title.len() > 55 {
            format!("{}...", &rfc.title[..55])
        } else {
            rfc.title.clone()
        };
        println!("{:<8} {:<60} {:?}", rfc.rfc_number, title, rfc.status);
    }

    Ok(())
}

/// 列出所有标签
async fn cmd_tags(_db: &db::DbPool) -> Result<()> {
    println!("⚠️  标签功能已从系统中移除：`tags` 命令不可用");
    Ok(())
}

/// 为RFC添加标签
async fn cmd_tag(_db: &db::DbPool, _rfc_number: i32, _tags: &str) -> Result<()> {
    println!("⚠️  标签功能已从系统中移除：无法为 RFC 添加标签");
    Ok(())
}

/// 查看RFC状态
async fn cmd_status(db: &db::DbPool, rfc_number: i32) -> Result<()> {
    let rfc = services::rfc::get_rfc_by_number(db, rfc_number).await?;
    
    // 统计翻译进度
    let stats: (i64, i64) = sqlx::query_as(
        "SELECT COUNT(*), COUNT(translated_text) FROM translations WHERE rfc_id = $1"
    )
    .bind(rfc.id)
    .fetch_one(db)
    .await?;

    println!("\n📄 RFC {} 状态\n", rfc_number);
    println!("标题: {}", rfc.title);
    println!("状态: {:?}", rfc.status);
    println!("创建时间: {}", rfc.created_at.format("%Y-%m-%d %H:%M:%S"));
    println!("更新时间: {}", rfc.updated_at.format("%Y-%m-%d %H:%M:%S"));
    
    // 标签功能已移除；不显示标签信息

    println!("\n📊 翻译进度: {}/{} ({:.1}%)", 
        stats.1, stats.0, 
        if stats.0 > 0 { (stats.1 as f64 / stats.0 as f64) * 100.0 } else { 0.0 }
    );

    Ok(())
}

/// 解析RFC结构
async fn cmd_parse(db: &db::DbPool, rfc_number: i32) -> Result<()> {
    println!("📝 正在解析 RFC {} 的结构...", rfc_number);

    let rfc = services::rfc::get_rfc_by_number(db, rfc_number).await?;
    parse_and_save_sections(db, &rfc).await?;

    println!("✅ 解析完成");

    Ok(())
}

/// 检查Ollama服务
async fn cmd_check_ollama(config: &Config) -> Result<()> {
    let ollama_url = config.ollama_url.as_ref()
        .ok_or_else(|| error::AppError::InternalError(
            "Ollama配置缺失。请设置 OLLAMA_URL 环境变量".to_string()
        ))?;
    let ollama_model = config.ollama_model.as_ref()
        .ok_or_else(|| error::AppError::InternalError(
            "Ollama模型配置缺失。请设置 OLLAMA_MODEL 环境变量".to_string()
        ))?;

    println!("🔍 检查Ollama服务状态...");
    println!("   URL: {}", ollama_url);
    println!("   模型: {}", ollama_model);
    println!();

    // 检查连接
    print!("📡 测试连接... ");
    std::io::Write::flush(&mut std::io::stdout()).ok();
    
    match services::ai::check_ollama_health(config).await {
        Ok(true) => {
            println!("✅ 成功");
            
            // 列出可用模型
            print!("📋 获取模型列表... ");
            std::io::Write::flush(&mut std::io::stdout()).ok();
            
            match services::ai::list_ollama_models(config).await {
                Ok(models) => {
                    println!("✅ 找到 {} 个模型\n", models.len());
                    println!("可用模型:");
                    for model in models {
                        let marker = if model.starts_with(ollama_model) || ollama_model.starts_with(&model) { 
                            " ✓ (当前)" 
                        } else { 
                            "" 
                        };
                        println!("   - {}{}", model, marker);
                    }
                    
                    // 已省略标签提取测试（标签功能已移除）
                }
                Err(e) => {
                    println!("❌ 失败");
                    println!("⚠️  无法获取模型列表: {}", e);
                }
            }
            
            println!("\n✅ Ollama 服务运行正常");
        }
        Ok(false) | Err(_) => {
            println!("❌ 失败");
            println!("\n❌ Ollama服务不可用");
            println!("\n💡 故障排查:");
            println!("   1. 检查 Ollama 是否运行:");
            println!("      ps aux | grep ollama");
            println!("   2. 启动 Ollama 服务:");
            println!("      ollama serve");
            println!("   3. 检查端口是否正确:");
            println!("      curl {}/api/tags", ollama_url);
            println!("   4. 下载所需模型:");
            println!("      ollama pull {}", ollama_model);
        }
    }

    Ok(())
}

/// 解析RFC文档并保存各段落
async fn parse_and_save_sections(db: &db::DbPool, rfc: &models::Rfc) -> Result<()> {
    let original_text = match &rfc.original_text {
        Some(text) => text,
        None => return Ok(()),
    };

    // 简单的段落解析逻辑
    let sections = parse_rfc_sections(original_text);

    for (section_id, content) in sections {
        // 检查是否已存在
        let existing = sqlx::query_as::<_, models::Translation>(
            "SELECT * FROM translations WHERE rfc_id = $1 AND section_id = $2"
        )
        .bind(rfc.id)
        .bind(&section_id)
        .fetch_optional(db)
        .await?;

        if existing.is_none() {
            sqlx::query(
                "INSERT INTO translations (rfc_id, section_id, original_text) VALUES ($1, $2, $3)"
            )
            .bind(rfc.id)
            .bind(&section_id)
            .bind(&content)
            .execute(db)
            .await?;
        }
    }

    // 更新RFC状态
    services::rfc::update_rfc_status(db, rfc.id, "parsing".to_string()).await?;

    Ok(())
}

/// 解析RFC文本为段落
fn parse_rfc_sections(text: &str) -> Vec<(String, String)> {
    let mut sections = Vec::new();
    let mut current_section = String::new();
    let mut current_content = Vec::new();
    let mut section_counter = 0;

    // 正则匹配章节标题
    let section_pattern = regex::Regex::new(r"^(\d+\.[\d.]*)\s+(.+)$").unwrap();

    for line in text.lines() {
        // 跳过页眉页脚等
        if line.contains("[Page") || line.trim().is_empty() && current_content.is_empty() {
            continue;
        }

        // 检测章节标题
        if let Some(caps) = section_pattern.captures(line.trim()) {
            // 保存上一个段落
            if !current_content.is_empty() {
                let content = current_content.join("\n").trim().to_string();
                if !content.is_empty() && content.len() > 50 {
                    sections.push((current_section.clone(), content));
                }
                current_content.clear();
            }
            
            current_section = caps.get(1).unwrap().as_str().to_string();
        } else {
            current_content.push(line.to_string());
        }

        // 当段落过长时分割
        if current_content.len() > 50 {
            let content = current_content.join("\n").trim().to_string();
            if !content.is_empty() {
                section_counter += 1;
                let section_id = if current_section.is_empty() {
                    format!("p{}", section_counter)
                } else {
                    format!("{}.p{}", current_section, section_counter)
                };
                sections.push((section_id, content));
            }
            current_content.clear();
        }
    }

    // 保存最后一个段落
    if !current_content.is_empty() {
        let content = current_content.join("\n").trim().to_string();
        if !content.is_empty() && content.len() > 50 {
            section_counter += 1;
            let section_id = if current_section.is_empty() {
                format!("p{}", section_counter)
            } else {
                format!("{}.p{}", current_section, section_counter)
            };
            sections.push((section_id, content));
        }
    }

    sections
}

/// 自动同步RFC
///
/// 使用生产者/消费者流水线：
///   生产者任务 —— 依次获取并解析 RFC（网络 I/O），完成后推入有界通道（容量 10）
///   消费者（主任务）—— 持续从通道取出 RFC 进行翻译（GPU 运算）
/// 两者并发运行，GPU 不再因等待下载而空闲。
async fn cmd_sync(
    db: &db::DbPool,
    config: &Config,
    start: i32,
    end: Option<i32>,
    skip_translate: bool,
    _concurrent: usize,
) -> Result<()> {
    println!("🔄 开始自动同步 RFC...");
    println!("📌 起始编号: {}", start);

    // 确定结束编号
    let end_number = if let Some(e) = end {
        println!("📌 结束编号: {}", e);
        e
    } else {
        println!("📌 结束编号: 自动检测（上限 9999）");
        9999
    };

    if start > end_number {
        println!("❌ 起始编号不能大于结束编号");
        return Ok(());
    }

    // 共享统计（生产者与消费者均可更新）
    let stats = Arc::new(Mutex::new(SyncStats {
        total: 0,
        added: 0,
        skipped: 0,
        translated: 0,
        failed: 0,
    }));

    // ── 翻译队列：容量 10 ──────────────────────────────────────────────────────
    // 生产者最多领先消费者（翻译器）10 个 RFC，确保 GPU 持续有工作可做。
    let (tx, mut rx) = tokio::sync::mpsc::channel::<i32>(10);

    // 克隆，供生产者任务使用
    let db_fetch = db.clone();
    let config_fetch = config.clone();
    let stats_fetch = Arc::clone(&stats);

    println!("\n🚀 流水线启动（获取与翻译并发运行）...\n");

    // ── 生产者任务：获取 RFC 并写入数据库 ────────────────────────────────────
    let fetch_handle = tokio::spawn(async move {
        // 一次性读取已有 RFC 编号，避免在循环内重复查询
        let existing_rfcs: Vec<i32> =
            sqlx::query_scalar("SELECT rfc_number FROM rfcs")
                .fetch_all(&db_fetch)
                .await
                .unwrap_or_default();

        for rfc_number in start..=end_number {
            stats_fetch.lock().unwrap().total += 1;

            if existing_rfcs.contains(&rfc_number) {
                // 检查该 RFC 是否还有未翻译的段落（包括翻译中断的情况）
                let untranslated_count: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM translations t \
                     JOIN rfcs r ON t.rfc_id = r.id \
                     WHERE r.rfc_number = $1 AND t.translated_text IS NULL",
                )
                .bind(rfc_number)
                .fetch_one(&db_fetch)
                .await
                .unwrap_or(0);

                if untranslated_count == 0 || skip_translate {
                    println!(
                        "⏭️  RFC {} 已存在{}，跳过",
                        rfc_number,
                        if untranslated_count == 0 { "且已翻译完成" } else { "" }
                    );
                    stats_fetch.lock().unwrap().skipped += 1;
                    continue;
                }

                // 已存在但有未翻译段落（含翻译中的）→ 推入翻译队列
                println!("📥 RFC {} 已存在但有 {} 个未翻译段落，推入翻译队列", rfc_number, untranslated_count);
                if tx.send(rfc_number).await.is_err() {
                    break; // 消费者已关闭
                }
            } else {
                // 从 IETF 获取 RFC 文本并解析段落
                println!("⬇️  获取 RFC {}...", rfc_number);
                let rfc = match services::rfc::fetch_rfc(&db_fetch, &config_fetch, rfc_number).await {
                    Ok(r) => r,
                    Err(e) => {
                        let msg = e.to_string();
                        if msg.contains("404") || msg.contains("Not Found") {
                            stats_fetch.lock().unwrap().skipped += 1;
                        } else {
                            println!("❌ RFC {} 获取失败: {}", rfc_number, e);
                            stats_fetch.lock().unwrap().failed += 1;
                        }
                        continue;
                    }
                };

                if let Err(e) = parse_and_save_sections(&db_fetch, &rfc).await {
                    println!("❌ RFC {} 解析失败: {}", rfc_number, e);
                    stats_fetch.lock().unwrap().failed += 1;
                    continue;
                }

                stats_fetch.lock().unwrap().added += 1;
                println!("✅ RFC {} 获取完成，推入翻译队列", rfc_number);

                if !skip_translate {
                    if tx.send(rfc_number).await.is_err() {
                        break; // 消费者已关闭
                    }
                }

                // 轻微速率限制，避免对 IETF 服务器造成压力
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            }
        }
        // tx 在此被丢弃，通道关闭，消费者的 while-let 循环随即退出
    });

    // ── 消费者：持续从队列取出 RFC 进行翻译，GPU 不空闲 ─────────────────────
    while let Some(rfc_number) = rx.recv().await {
        println!("🌐 翻译 RFC {}...", rfc_number);
        match cmd_translate(db, config, rfc_number, false).await {
            Ok(_) => {
                stats.lock().unwrap().translated += 1;
                println!("✅ RFC {} 翻译完成", rfc_number);
            }
            Err(e) => {
                println!("❌ RFC {} 翻译失败: {}", rfc_number, e);
                stats.lock().unwrap().failed += 1;
            }
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }

    // 等待生产者任务彻底结束
    let _ = fetch_handle.await;

    // 打印最终统计
    let s = stats.lock().unwrap();
    println!("\n📊 同步统计:");
    println!("   总计:    {}", s.total);
    println!("   ✅ 新增:  {}", s.added);
    println!("   📝 翻译:  {}", s.translated);
    println!("   ⏭️  跳过:  {}", s.skipped);
    println!("   ❌ 失败:  {}", s.failed);

    Ok(())
}

struct SyncStats {
    total: usize,
    added: usize,
    skipped: usize,
    translated: usize,
    failed: usize,
}
