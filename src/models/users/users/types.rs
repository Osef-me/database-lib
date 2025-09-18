use chrono::NaiveDateTime;
use serde_json::Value;
use validator::Validate;

#[derive(Debug, Clone, sqlx::FromRow, Validate)]
pub struct UsersRow {
    /// Discord ID of the user (primary key).
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "Discord ID must be positive"))]
    pub discord_id: i64,

    /// Username of the user.
    /// Optional field, can be None.
    pub username: Option<String>,

    /// Timestamp when the user was created.
    pub created_at: Option<NaiveDateTime>,

    /// Roles assigned to the user stored as JSON array.
    /// Defaults to ["user"].
    pub roles: Value,
}
