use axum::{
    Json,
    extract::{Path, Query, State},
};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::booking::{
    Booking, BookingListParams, BookingStatus, CreateBooking, UpdateBookingStatus,
};
use crate::models::{ApiResponse, WhitelistEntry};
use crate::state::AppState;
use crate::types::PhoneNumber;

/// POST /api/bookings - Submit a new booking request
pub async fn create(
    State(state): State<AppState>,
    Json(input): Json<CreateBooking>,
) -> Result<Json<ApiResponse<Booking>>, AppError> {
    let phone = PhoneNumber::parse_north_american(&input.caller_phone)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    if let Some(ref email) = input.caller_email
        && !email.contains('@')
    {
        return Err(AppError::BadRequest("invalid email format".to_string()));
    }

    let booking = sqlx::query_as::<_, Booking>(
        "INSERT INTO bookings (caller_name, caller_phone, caller_email, call_reason)
         VALUES ($1, $2, $3, $4)
         RETURNING id, caller_name, caller_phone, caller_email, call_reason, status, created_at, resolved_at, whitelist_entry_id",
    )
    .bind(&input.caller_name)
    .bind(phone)
    .bind(&input.caller_email)
    .bind(&input.call_reason)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(booking)))
}

/// GET /api/bookings - List bookings, optionally filtered by status
pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<BookingListParams>,
) -> Result<Json<ApiResponse<Vec<Booking>>>, AppError> {
    let bookings = match params.status {
        Some(status) => {
            sqlx::query_as::<_, Booking>(
                "SELECT id, caller_name, caller_phone, caller_email, call_reason, status, created_at, resolved_at, whitelist_entry_id
                 FROM bookings WHERE status = $1 ORDER BY created_at DESC",
            )
            .bind(status)
            .fetch_all(&state.pool)
            .await?
        }
        None => {
            sqlx::query_as::<_, Booking>(
                "SELECT id, caller_name, caller_phone, caller_email, call_reason, status, created_at, resolved_at, whitelist_entry_id
                 FROM bookings ORDER BY created_at DESC",
            )
            .fetch_all(&state.pool)
            .await?
        }
    };

    Ok(Json(ApiResponse::success(bookings)))
}

/// GET /api/bookings/:id - Get a single booking
pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Booking>>, AppError> {
    let booking = sqlx::query_as::<_, Booking>(
        "SELECT id, caller_name, caller_phone, caller_email, call_reason, status, created_at, resolved_at, whitelist_entry_id
         FROM bookings WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(ApiResponse::success(booking)))
}

/// PATCH /api/bookings/:id - Approve or deny a booking
pub async fn update_status(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateBookingStatus>,
) -> Result<Json<ApiResponse<Booking>>, AppError> {
    let existing = sqlx::query_as::<_, Booking>(
        "SELECT id, caller_name, caller_phone, caller_email, call_reason, status, created_at, resolved_at, whitelist_entry_id
         FROM bookings WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    if !matches!(existing.status, BookingStatus::Pending) {
        return Err(AppError::BadRequest("booking already resolved".to_string()));
    }

    match input.status {
        BookingStatus::Approved => {
            let mut tx = state.pool.begin().await?;

            let entry = sqlx::query_as::<_, WhitelistEntry>(
                "INSERT INTO whitelist_entries (phone_number, name, reason, is_permanent)
                 VALUES ($1, $2, $3, false)
                 RETURNING id, phone_number, name, reason, created_at, expires_at, is_permanent",
            )
            .bind(&existing.caller_phone)
            .bind(&existing.caller_name)
            .bind(&existing.call_reason)
            .fetch_one(&mut *tx)
            .await?;

            let booking = sqlx::query_as::<_, Booking>(
                "UPDATE bookings
                 SET status = 'approved', resolved_at = NOW(), whitelist_entry_id = $1
                 WHERE id = $2
                 RETURNING id, caller_name, caller_phone, caller_email, call_reason, status, created_at, resolved_at, whitelist_entry_id",
            )
            .bind(entry.id)
            .bind(id)
            .fetch_one(&mut *tx)
            .await?;

            tx.commit().await?;

            Ok(Json(ApiResponse::success(booking)))
        }
        BookingStatus::Denied => {
            let booking = sqlx::query_as::<_, Booking>(
                "UPDATE bookings SET status = 'denied', resolved_at = NOW()
                 WHERE id = $1
                 RETURNING id, caller_name, caller_phone, caller_email, call_reason, status, created_at, resolved_at, whitelist_entry_id",
            )
            .bind(id)
            .fetch_one(&state.pool)
            .await?;

            Ok(Json(ApiResponse::success(booking)))
        }
        BookingStatus::Pending => Err(AppError::BadRequest(
            "cannot set status to pending".to_string(),
        )),
    }
}
