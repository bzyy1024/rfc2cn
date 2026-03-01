use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Rfc {
    pub id: i32,
    pub rfc_number: i32,
    pub title: String,
    pub original_text: Option<String>,
    pub parsed_structure: Option<serde_json::Value>,
    pub status: String,
    pub r#abstract: Option<String>,
    pub authors: Option<Vec<String>>,
    pub publish_date: Option<NaiveDate>,
    pub obsoletes: Option<Vec<i32>>,
    pub obsoleted_by: Option<Vec<i32>>,
    pub updates: Option<Vec<i32>>,
    pub updated_by: Option<Vec<i32>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// RFC列表项（不含原文，用于列表展示）
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RfcListItem {
    pub id: i32,
    pub rfc_number: i32,
    pub title: String,
    pub status: String,
    pub r#abstract: Option<String>,
    pub publish_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRfcRequest {
    pub rfc_number: i32,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct FetchRfcRequest {
    pub rfc_number: i32,
}

#[derive(Debug, Deserialize)]
pub struct SearchRfcQuery {
    pub q: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct SearchRfcResponse {
    pub rfcs: Vec<RfcListItem>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}
