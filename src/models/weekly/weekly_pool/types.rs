use chrono::NaiveDateTime;
use validator::Validate;

#[derive(Debug, Clone, sqlx::FromRow, Validate)]
pub struct WeeklyPoolRow {
    /// Unique identifier for the weekly pool record.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    /// Reference to the beatmap this pool entry refers to.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "Beatmap ID must be positive"))]
    pub beatmap_id: i32,

    /// Reference to the weekly challenge this pool belongs to.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "Weekly ID must be positive"))]
    pub weekly_id: i32,

    /// Number of votes for this beatmap in the pool.
    /// Must be a non-negative integer (≥ 0).
    #[validate(range(min = 0, message = "Vote count must be non-negative"))]
    pub vote_count: i32,

    /// Timestamp when the weekly pool entry was created.
    pub created_at: Option<NaiveDateTime>,
}
