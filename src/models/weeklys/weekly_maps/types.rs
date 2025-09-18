use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use validator::Validate;

#[derive(Debug, Clone, sqlx::FromRow, Validate)]
pub struct WeeklyMapsRow {
    /// Unique identifier for the weekly maps record.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    /// Reference to the beatmap this weekly map refers to.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "Beatmap ID must be positive"))]
    pub beatmap_id: i32,

    /// Reference to the weekly challenge this map belongs to.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "Weekly ID must be positive"))]
    pub weekly_id: i32,

    /// Maximum rate multiplier allowed for this map.
    /// Must be between 0.5 and 10.0 (decimal(4,2) constraint).
    pub max_rate: BigDecimal,

    /// Timestamp when the weekly map was created.
    pub created_at: Option<NaiveDateTime>,
}
