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
    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: String,

    /// End date of the weekly challenge.
    /// Can be null if the weekly is ongoing.
    pub end_at: Option<NaiveDateTime>,

    /// Start date of the weekly challenge.
    /// Can be null if the weekly hasn't started yet.
    pub start_at: Option<NaiveDateTime>,

    /// Timestamp when the weekly challenge was created.
    pub created_at: Option<NaiveDateTime>,
}
