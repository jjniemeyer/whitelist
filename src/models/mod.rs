pub mod booking;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::types::PhoneNumber;

#[derive(Debug, Serialize, FromRow)]
pub struct WhitelistEntry {
    pub id: Uuid,
    pub phone_number: PhoneNumber,
    pub name: String,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_permanent: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateWhitelistEntry {
    pub phone_number: String,
    pub name: String,
    pub reason: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_permanent: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.to_string()),
        }
    }
}
