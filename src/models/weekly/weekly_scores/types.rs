use chrono::NaiveDateTime;
use validator::Validate;

#[derive(Debug, Clone, sqlx::FromRow, Validate)]
pub struct WeeklyScoresRow {
    /// Unique identifier for the weekly scores record.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    /// Reference to the weekly challenge this score belongs to.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "Weekly ID must be positive"))]
    pub weekly_id: i32,

    /// Reference to the score record.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "Score ID must be positive"))]
    pub score_id: i32,

    /// Timestamp when the weekly score was created.
    pub created_at: Option<NaiveDateTime>,
}

