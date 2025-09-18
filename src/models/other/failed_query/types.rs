use chrono::NaiveDateTime;
use validator::Validate;

use crate::utils::HASH_REGEX;

#[derive(Debug, Clone, sqlx::FromRow, Validate)]
pub struct FailedQueryRow {
    /// Unique identifier for the failed query record.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    /// Hash of the failed query.
    /// Must be between 1 and 255 characters.
    #[validate(length(
        min = 1,
        max = 255,
        message = "Hash must be between 1 and 255 characters"
    ))]
    #[validate(regex(path = *HASH_REGEX))]
    pub hash: String,

    /// Timestamp when the failed query was created.
    pub created_at: Option<NaiveDateTime>,
}
