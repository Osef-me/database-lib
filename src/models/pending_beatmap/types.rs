use chrono::NaiveDateTime;
use validator::Validate;

use crate::utils::HASH_REGEX;

#[derive(Debug, Clone, Validate)]
pub struct PendingBeatmapRow {
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

    #[validate(range(min = 1, message = "Osu ID must be positive"))]
    pub osu_id: Option<i32>,

    pub created_at: Option<NaiveDateTime>,
}
