use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

pub async fn health_check() -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "status": "healthy",
            "service": "rfc2cn-backend",
            "version": env!("CARGO_PKG_VERSION")
        })),
    )
}
