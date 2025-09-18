use chrono::NaiveDateTime;
use validator::Validate;

#[derive(Debug, Clone, sqlx::FromRow, Validate)]
pub struct BansRow {
    /// Unique identifier for the ban record.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    /// Discord ID of the banned user.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "Discord ID must be positive"))]
    pub discord_id: Option<i64>,

    /// Optional reason for the ban.
    pub reason: Option<String>,

    /// Timestamp when the user was banned.
    pub banned_at: Option<NaiveDateTime>,
}
