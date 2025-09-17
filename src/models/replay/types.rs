use chrono::NaiveDateTime;
use sqlx::FromRow;
use validator::Validate;

use crate::utils::HASH_REGEX;

#[derive(Debug, Clone, FromRow, Validate)]
pub struct Replay {
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    #[validate(length(
        min = 1,
        max = 255,
        message = "Hash must be between 1 and 255 characters"
    ))]
    #[validate(regex(
        path = "*HASH_REGEX",
        message = "Hash must contain only alphanumeric characters"
    ))]
    pub hash: String,

    pub replay_available: bool,

    #[validate(length(
        min = 1,
        max = 500,
        message = "Replay path must be between 1 and 500 characters"
    ))]
    pub replay_path: String,

    pub created_at: Option<NaiveDateTime>,
}
