use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),

    #[error("未找到: {0}")]
    NotFound(String),

    #[error("未授权")]
    Unauthorized,

    #[error("验证失败: {0}")]
    ValidationError(String),

    #[error("内部错误: {0}")]
    InternalError(String),

    #[error("外部API错误: {0}")]
    ExternalApiError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(ref e) => {
                tracing::error!("数据库错误: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "数据库错误".to_string())
            }
            AppError::NotFound(ref msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "未授权".to_string()),
            AppError::ValidationError(ref msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::InternalError(ref msg) => {
                tracing::error!("内部错误: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "内部服务器错误".to_string())
            }
            AppError::ExternalApiError(ref msg) => {
                tracing::error!("外部API错误: {}", msg);
                (StatusCode::BAD_GATEWAY, "外部服务错误".to_string())
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
