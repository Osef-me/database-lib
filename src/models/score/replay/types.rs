use chrono::NaiveDateTime;
use validator::Validate;

#[derive(Debug, Clone, sqlx::FromRow, Validate)]
pub struct ReplayRow {
    /// Unique identifier for the replay record.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    /// Replay data as binary content.
    /// Must not be empty.
    #[validate(length(min = 1, message = "Replay data cannot be empty"))]
    pub replay_data: Vec<u8>,

    /// Timestamp when the replay was created.
    pub created_at: Option<NaiveDateTime>,
}

