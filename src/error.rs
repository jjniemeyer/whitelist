use crate::models::ApiResponse;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("not found")]
    NotFound,

    #[error("bad request: {0}")]
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            }
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
        };

        let body = Json(ApiResponse::<()>::error(message));
        (status, body).into_response()
    }
}
