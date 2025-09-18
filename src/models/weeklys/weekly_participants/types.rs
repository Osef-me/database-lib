use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use validator::Validate;

#[derive(Debug, Clone, sqlx::FromRow, Validate)]
pub struct WeeklyParticipantsRow {
    /// Unique identifier for the weekly participants record.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    /// Discord ID of the participant.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "User ID must be positive"))]
    pub user_id: i64,

    /// Reference to the weekly challenge this participant belongs to.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "Weekly ID must be positive"))]
    pub weekly_id: i32,

    /// Overall Performance (OP) points for this participant.
    /// Must be a non-negative decimal (≥ 0).
    pub op: BigDecimal,

    /// Final rank of the participant (computed at the end).
    /// Must be a positive integer (≥ 1) if set.
    #[validate(range(min = 1, message = "Final rank must be positive"))]
    pub final_rank: Option<i32>,

    /// Timestamp when the participant joined the weekly challenge.
    pub created_at: Option<NaiveDateTime>,
}
