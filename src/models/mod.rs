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

// --- Pagination ---

const MAX_PER_PAGE: u32 = 100;

fn default_page() -> u32 {
    1
}

fn default_per_page() -> u32 {
    20
}

#[derive(Debug, Deserialize)]
pub struct Pagination {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

impl Pagination {
    /// Validate pagination parameters.
    /// Returns Err with a human-readable message if page or per_page is 0.
    /// Clamps per_page to MAX_PER_PAGE silently.
    pub fn validate(mut self) -> Result<Self, String> {
        if self.page == 0 {
            return Err("page must be >= 1".to_string());
        }
        if self.per_page == 0 {
            return Err("per_page must be >= 1".to_string());
        }
        if self.per_page > MAX_PER_PAGE {
            self.per_page = MAX_PER_PAGE;
        }
        Ok(self)
    }

    pub fn offset(&self) -> i64 {
        ((self.page - 1) * self.per_page) as i64
    }

    pub fn limit(&self) -> i64 {
        self.per_page as i64
    }
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub total: i64,
    pub page: u32,
    pub per_page: u32,
    pub data: Vec<T>,
}
