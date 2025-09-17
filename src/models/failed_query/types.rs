use crate::utils::HASH_REGEX;
use chrono::NaiveDateTime;
use validator::Validate;

/// Represents a failed query record in the database.
///
/// This struct is used to track queries that have failed for various reasons,
/// typically used for monitoring, debugging, or retry mechanisms.
///
/// # Fields
///
/// * `id` - Unique identifier for the failed query record (must be positive)
/// * `hash` - Hash identifier of the failed query (1-255 alphanumeric characters)
/// * `created_at` - Timestamp when the failed query was recorded (optional)
///
/// # Validation
///
/// The struct implements validation rules:
/// - `id` must be a positive integer (≥ 1)
/// - `hash` must be between 1 and 255 characters long
/// - `hash` must match the alphanumeric pattern defined in `HASH_REGEX`
///
/// # Examples
///
/// ```rust
/// use crate::models::failed_query::FailedQuery;
/// use chrono::NaiveDateTime;
///
/// // Create a valid failed query record
/// let failed_query = FailedQuery {
///     id: 1,
///     hash: "abc123def456".to_string(),
///     created_at: Some(NaiveDateTime::from_timestamp_opt(1640995200, 0).unwrap()),
/// };
///
/// // Validate the record
/// assert!(failed_query.validate().is_ok());
/// ```
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
