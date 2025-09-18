use chrono::NaiveDateTime;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, sqlx::FromRow, Validate)]
pub struct NewUsersRow {
    /// Discord ID of the new user (primary key).
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "Discord ID must be positive"))]
    pub discord_id: i64,

    /// Optional username of the new user.
    pub username: Option<String>,

    /// Unique validation token sent via Discord DM.
    pub token: Uuid,

    /// Timestamp when the new user record was created.
    pub created_at: Option<NaiveDateTime>,
}

