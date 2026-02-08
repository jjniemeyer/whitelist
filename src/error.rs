use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use crate::models::ApiResponse;

#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    NotImplemented,
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            }
            AppError::NotImplemented => {
                (StatusCode::NOT_IMPLEMENTED, "Not implemented")
            }
            AppError::NotFound => {
                (StatusCode::NOT_FOUND, "Not found")
            }
        };

        let body = Json(ApiResponse::<()>::error(message));
        (status, body).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::Database(e)
    }
}
