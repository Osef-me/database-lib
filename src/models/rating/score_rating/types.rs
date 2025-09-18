use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use validator::Validate;

#[derive(Debug, Clone, sqlx::FromRow, Validate)]
pub struct ScoreRatingRow {
    /// Unique identifier for the score rating record.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    /// Reference to the score record this rating applies to.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "Score ID must be positive"))]
    pub score_id: i32,

    /// Rating value for the score.
    /// Must be a positive decimal value.
    pub rating: BigDecimal,

    /// Type of rating system used.
    /// Must be one of: 'osu', 'etterna', 'quaver', 'malody', 'interlude'.
    #[validate(custom(function = "validate_rating_type"))]
    pub rating_type: String,

    /// Timestamp when the score rating was created.
    pub created_at: Option<NaiveDateTime>,
}

fn validate_rating_type(rating_type: &str) -> Result<(), validator::ValidationError> {
    match rating_type {
        "osu" | "etterna" | "quaver" | "malody" | "interlude" => Ok(()),
        _ => Err(validator::ValidationError::new("invalid_rating_type")),
    }
}
