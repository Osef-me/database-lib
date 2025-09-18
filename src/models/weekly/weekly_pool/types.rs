use chrono::NaiveDateTime;
use validator::Validate;

#[derive(Debug, Clone, sqlx::FromRow, Validate)]
pub struct WeeklyPoolRow {
    /// Unique identifier for the weekly pool record.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    /// Reference to the weekly challenge this pool belongs to.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "Weekly ID must be positive"))]
    pub weekly_id: i32,

    /// Name of the pool.
    /// Must be between 1 and 255 characters.
    #[validate(length(min = 1, max = 255, message = "Name must be between 1 and 255 characters"))]
    pub name: String,

    /// Description of the pool.
    /// Optional field, can be None.
    pub description: Option<String>,

    /// Timestamp when the weekly pool was created.
    pub created_at: Option<NaiveDateTime>,
}

