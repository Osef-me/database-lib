use chrono::NaiveDateTime;
use validator::Validate;

#[derive(Debug, Clone, sqlx::FromRow, Validate)]
pub struct WeeklyMapsRow {
    /// Unique identifier for the weekly maps record.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    /// Reference to the weekly pool this map belongs to.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "Weekly pool ID must be positive"))]
    pub weekly_pool_id: i32,

    /// Reference to the beatmap this weekly map refers to.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "Beatmap ID must be positive"))]
    pub beatmap_id: i32,

    /// Timestamp when the weekly map was created.
    pub created_at: Option<NaiveDateTime>,
}

