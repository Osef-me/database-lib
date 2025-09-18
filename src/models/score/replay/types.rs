use chrono::NaiveDateTime;
use validator::Validate;

use crate::utils::HASH_REGEX;

#[derive(Debug, Clone, sqlx::FromRow, Validate)]
pub struct ReplayRow {
    /// Unique identifier for the replay record.
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    #[validate(regex(path = *HASH_REGEX))]
    pub replay_hash: String,

    /// Must be a boolean.
    pub replay_available: bool,

    /// Must be a string.
    #[validate(length(min = 1, message = "Replay path cannot be empty"))]
    pub replay_path: String,

    /// Must be a timestamp.
    pub created_at: Option<NaiveDateTime>,
}
