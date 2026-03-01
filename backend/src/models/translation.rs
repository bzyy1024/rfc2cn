use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Translation {
    pub id: i32,
    pub rfc_id: i32,
    pub section_id: String,
    pub original_text: String,
    pub translated_text: Option<String>,
    pub reviewed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TranslationTask {
    pub id: Uuid,
    pub rfc_id: i32,
    pub status: String,
    pub progress: i32,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTranslationTaskRequest {
    pub rfc_number: i32,
}
