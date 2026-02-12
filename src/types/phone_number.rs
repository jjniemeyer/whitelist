use serde::{Deserialize, Serialize};
use sqlx::Database;

#[derive(Debug, Clone, thiserror::Error)]
#[error("invalid phone number: {0}")]
pub struct PhoneNumberError(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    /// Parse a North American (NANP) phone number into E.164 format.
    ///
    /// Accepts formats like:
    ///   555-234-5678, (555) 234-5678, 5552345678, +15552345678
    ///
    /// Returns the number stored as +1NXXNXXXXXX.
    pub fn parse_north_american(input: &str) -> Result<Self, PhoneNumberError> {
        // Strip everything except digits
        let digits: String = input.chars().filter(|c| c.is_ascii_digit()).collect();

        // Normalize to 10 digits (strip leading country code 1)
        let ten = if digits.len() == 11 && digits.starts_with('1') {
            &digits[1..]
        } else if digits.len() == 10 {
            &digits
        } else {
            return Err(PhoneNumberError(format!(
                "expected 10-digit NANP number, got {} digits",
                digits.len()
            )));
        };

        let npa = &ten[0..3]; // area code
        let nxx = &ten[3..6]; // exchange

        // NANP rules: area code and exchange must start with 2-9
        if npa.starts_with('0') || npa.starts_with('1') {
            return Err(PhoneNumberError(
                "area code must start with 2-9".to_string(),
            ));
        }
        if nxx.starts_with('0') || nxx.starts_with('1') {
            return Err(PhoneNumberError(
                "exchange must start with 2-9".to_string(),
            ));
        }

        Ok(Self(format!("+1{ten}")))
    }

}

// sqlx Type/Encode/Decode — delegate to the inner String
impl<DB: Database> sqlx::Type<DB> for PhoneNumber
where
    String: sqlx::Type<DB>,
{
    fn type_info() -> <DB as Database>::TypeInfo {
        <String as sqlx::Type<DB>>::type_info()
    }

    fn compatible(ty: &<DB as Database>::TypeInfo) -> bool {
        <String as sqlx::Type<DB>>::compatible(ty)
    }
}

impl<'q, DB: Database> sqlx::Encode<'q, DB> for PhoneNumber
where
    String: sqlx::Encode<'q, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
        self.0.encode_by_ref(buf)
    }
}

impl<'r, DB: Database> sqlx::Decode<'r, DB> for PhoneNumber
where
    String: sqlx::Decode<'r, DB>,
{
    fn decode(
        value: <DB as Database>::ValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let s = <String as sqlx::Decode<'r, DB>>::decode(value)?;
        Ok(Self(s))
    }
}
