use chrono::NaiveDateTime;
use validator::Validate;

use crate::utils::HASH_REGEX;

#[derive(Debug, Clone, sqlx::FromRow, Validate)]
pub struct PendingBeatmapRow {
    /// Unique identifier for the pending beatmap record.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    /// Osu hash of the pending beatmap.
    /// Must be between 1 and 255 characters.
    #[validate(length(
        min = 1,
        max = 255,
        message = "Osu hash must be between 1 and 255 characters"
    ))]
    #[validate(regex(
        path = "*HASH_REGEX",
        message = "Hash must contain only alphanumeric characters"
    ))]
    pub osu_hash: String,

    /// Osu ID of the pending beatmap.
    /// Optional field, can be None.
    pub osu_id: Option<i32>,

    /// Timestamp when the pending beatmap was created.
    pub created_at: Option<NaiveDateTime>,
}
