use chrono::NaiveDateTime;
use validator::Validate;

#[derive(Debug, Clone, sqlx::FromRow, Validate)]
pub struct BeatmapRatingRow {
    /// Unique identifier for the beatmap rating record.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    /// Reference to the rates record this rating applies to.
    /// Optional field, can be None.
    pub rates_id: Option<i32>,

    /// Rating value for the beatmap.
    /// Must be a positive decimal value.
    #[validate(range(min = 0.01, message = "Rating must be positive"))]
    pub rating: f64,

    /// Type of rating system used.
    /// Must be one of: 'osu', 'etterna', 'quaver', 'malody', 'interlude'.
    #[validate(custom = "validate_rating_type")]
    pub rating_type: String,

    /// Timestamp when the rating was created.
    pub created_at: Option<NaiveDateTime>,
}

fn validate_rating_type(rating_type: &str) -> Result<(), validator::ValidationError> {
    match rating_type {
        "osu" | "etterna" | "quaver" | "malody" | "interlude" => Ok(()),
        _ => Err(validator::ValidationError::new("invalid_rating_type")),
    }
}
