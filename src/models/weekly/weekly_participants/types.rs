use chrono::NaiveDateTime;
use validator::Validate;

#[derive(Debug, Clone, sqlx::FromRow, Validate)]
pub struct WeeklyParticipantsRow {
    /// Unique identifier for the weekly participants record.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    /// Reference to the weekly challenge this participant belongs to.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "Weekly ID must be positive"))]
    pub weekly_id: i32,

    /// Discord ID of the participant.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "User ID must be positive"))]
    pub user_id: i64,

    /// Timestamp when the participant joined the weekly challenge.
    pub created_at: Option<NaiveDateTime>,
}

