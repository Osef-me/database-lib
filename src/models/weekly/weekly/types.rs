use chrono::NaiveDateTime;
use validator::Validate;

#[derive(Debug, Clone, sqlx::FromRow, Validate)]
pub struct WeeklyRow {
    /// Unique identifier for the weekly record.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    /// Name of the weekly challenge.
    /// Must be between 1 and 255 characters.
    #[validate(length(min = 1, max = 255, message = "Name must be between 1 and 255 characters"))]
    pub name: String,

    /// Description of the weekly challenge.
    /// Optional field, can be None.
    pub description: Option<String>,

    /// Start date of the weekly challenge.
    pub start_date: NaiveDateTime,

    /// End date of the weekly challenge.
    pub end_date: NaiveDateTime,

    /// Whether the weekly challenge is currently active.
    pub is_active: bool,

    /// Timestamp when the weekly challenge was created.
    pub created_at: Option<NaiveDateTime>,
}

