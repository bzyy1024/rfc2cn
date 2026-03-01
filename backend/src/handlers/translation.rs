use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::error::Result;
use crate::models::{CreateTranslationTaskRequest, TranslationTask};
use crate::services;

use super::AppState;

pub async fn create_translation_task(
    State(state): State<AppState>,
    Json(payload): Json<CreateTranslationTaskRequest>,
) -> Result<(StatusCode, Json<TranslationTask>)> {
    let task = services::translation::create_translation_task(
        &state.db,
        &state.config,
        payload.rfc_number,
    )
    .await?;

    Ok((StatusCode::CREATED, Json(task)))
}

pub async fn get_translation_status(
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
) -> Result<Json<TranslationTask>> {
    let task = services::translation::get_translation_task(&state.db, task_id).await?;
    Ok(Json(task))
}
