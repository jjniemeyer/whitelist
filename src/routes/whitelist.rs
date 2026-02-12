use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{ApiResponse, CreateWhitelistEntry, WhitelistEntry};
use crate::state::AppState;
use crate::types::PhoneNumber;

/// GET /api/whitelist - List all whitelist entries
pub async fn list(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<WhitelistEntry>>>, AppError> {
    let entries = sqlx::query_as::<_, WhitelistEntry>(
        "SELECT id, phone_number, name, reason, created_at, expires_at, is_permanent
         FROM whitelist_entries
         ORDER BY created_at DESC",
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(entries)))
}

/// GET /api/whitelist/{id} - Get a specific whitelist entry
pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<WhitelistEntry>>, AppError> {
    let entry = sqlx::query_as::<_, WhitelistEntry>(
        "SELECT id, phone_number, name, reason, created_at, expires_at, is_permanent
         FROM whitelist_entries
         WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(ApiResponse::success(entry)))
}

/// POST /api/whitelist - Create a new whitelist entry
pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateWhitelistEntry>,
) -> Result<Json<ApiResponse<WhitelistEntry>>, AppError> {
    let phone = PhoneNumber::parse_north_american(&payload.phone_number)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let is_permanent = payload.is_permanent.unwrap_or(false);

    let entry = sqlx::query_as::<_, WhitelistEntry>(
        "INSERT INTO whitelist_entries (phone_number, name, reason, expires_at, is_permanent)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id, phone_number, name, reason, created_at, expires_at, is_permanent",
    )
    .bind(phone)
    .bind(&payload.name)
    .bind(&payload.reason)
    .bind(payload.expires_at)
    .bind(is_permanent)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(entry)))
}

/// DELETE /api/whitelist/{id} - Delete a whitelist entry
pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let result = sqlx::query("DELETE FROM whitelist_entries WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(Json(ApiResponse::success(())))
}
