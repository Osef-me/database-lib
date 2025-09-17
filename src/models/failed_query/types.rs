use crate::utils::HASH_REGEX;
use chrono::NaiveDateTime;
use validator::Validate;

#[derive(Debug, Clone, sqlx::FromRow, Validate)]
pub struct FailedQuery {
    /// Unique identifier for the failed query record.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    /// Hash identifier of the failed query.
    /// Must be between 1 and 255 characters and contain only alphanumeric characters.
    #[validate(length(
        min = 1,
        max = 255,
        message = "Hash must be between 1 and 255 characters"
    ))]
    #[validate(regex(path = "*HASH_REGEX"))]
    pub hash: String,

    /// Timestamp when the failed query was recorded.
    /// This field is optional and can be `None` if the timestamp is not available.
    pub created_at: Option<NaiveDateTime>,
}
