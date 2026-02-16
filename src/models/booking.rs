use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::types::PhoneNumber;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "booking_status", rename_all = "lowercase")]
pub enum BookingStatus {
    Pending,
    Approved,
    Denied,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Booking {
    pub id: Uuid,
    pub caller_name: String,
    pub caller_phone: PhoneNumber,
    pub caller_email: Option<String>,
    pub call_reason: Option<String>,
    pub status: BookingStatus,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub whitelist_entry_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBooking {
    pub caller_name: String,
    pub caller_phone: String,
    pub caller_email: Option<String>,
    pub call_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBookingStatus {
    pub status: BookingStatus,
}

#[derive(Debug, Deserialize)]
pub struct BookingListParams {
    pub status: Option<BookingStatus>,
}
