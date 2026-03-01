use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::error::Result;
use crate::models::{Rfc, RfcListItem, SearchRfcQuery, SearchRfcResponse, Translation};
use crate::services;

use super::AppState;

/// 获取RFC列表
pub async fn list_rfcs(State(state): State<AppState>) -> Result<Json<Vec<RfcListItem>>> {
    let rfcs = services::rfc::list_rfcs(&state.db).await?;
    Ok(Json(rfcs))
}

/// 搜索RFC
pub async fn search_rfcs(
    State(state): State<AppState>,
    Query(query): Query<SearchRfcQuery>,
) -> Result<Json<SearchRfcResponse>> {
    let response = services::rfc::search_rfcs(&state.db, query).await?;
    Ok(Json(response))
}

/// 根据RFC编号获取详情
pub async fn get_rfc(
    State(state): State<AppState>,
    Path(rfc_number): Path<i32>,
) -> Result<Json<Rfc>> {
    let rfc = services::rfc::get_rfc_by_number(&state.db, rfc_number).await?;
    Ok(Json(rfc))
}

/// 获取RFC的所有翻译
pub async fn get_rfc_translations(
    State(state): State<AppState>,
    Path(rfc_number): Path<i32>,
) -> Result<Json<Vec<Translation>>> {
    let translations = services::rfc::get_rfc_translations(&state.db, rfc_number).await?;
    Ok(Json(translations))
}
