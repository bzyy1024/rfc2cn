use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod db;
mod error;
mod handlers;
mod models;
mod services;

use config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rfc2cn_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 加载配置
    let config = Config::from_env()?;
    tracing::info!("配置加载完成");
    
    // 显示 AI 配置状态
    match &config.ai_provider {
        Some(provider) => tracing::info!("AI提供商: {}", provider),
        None => tracing::warn!("AI翻译功能未启用（未配置 AI_PROVIDER）"),
    }

    // 初始化数据库连接池
    let db_pool = db::create_pool(&config.database_url).await?;
    tracing::info!("数据库连接成功");

    // 注意：数据库结构已通过 create_all_tables.sql 手动创建
    // 不需要运行自动迁移
    tracing::info!("数据库已就绪");

    // 配置CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 构建应用状态
    let app_state = handlers::AppState {
        db: db_pool,
        config: config.clone(),
    };

    // 构建路由
    let app = Router::new()
        .route("/", get(handlers::health::health_check))
        .route("/api/health", get(handlers::health::health_check))
        // RFC相关路由
        .route("/api/rfcs", get(handlers::rfc::list_rfcs))
        .route("/api/rfcs/search", get(handlers::rfc::search_rfcs))
        .route("/api/rfcs/:number", get(handlers::rfc::get_rfc))
        .route("/api/rfcs/:number/translations", get(handlers::rfc::get_rfc_translations))
        // （标签功能已移除）
        // 翻译相关路由
        .route("/api/translate/task", post(handlers::translation::create_translation_task))
        .route("/api/translate/status/:task_id", get(handlers::translation::get_translation_status))
        .layer(cors)
        .with_state(app_state);

    // 启动服务器
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    tracing::info!("服务器启动在 http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
