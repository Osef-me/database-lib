use chrono::NaiveDateTime;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, sqlx::FromRow, Validate)]
pub struct DeviceTokensRow {
    /// Unique token identifier (UUID).
    pub token: Uuid,

    /// Discord ID of the user who owns this device token.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "Discord ID must be positive"))]
    pub discord_id: i64,

    /// Optional device name identifier.
    pub device_name: Option<String>,

    /// Optional hardware identifier.
    pub hwid: Option<String>,

    /// Timestamp when the device token was created.
    pub created_at: Option<NaiveDateTime>,
}

