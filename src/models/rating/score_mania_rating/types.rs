use chrono::NaiveDateTime;
use validator::Validate;

#[derive(Debug, Clone, sqlx::FromRow, Validate)]
pub struct ScoreManiaRatingRow {
    /// Unique identifier for the score mania rating record.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    /// Reference to the score rating record this mania rating applies to.
    /// Optional field, can be None.
    pub rating_id: Option<i32>,

    /// Stream difficulty rating.
    /// Must be a non-negative decimal value (≥ 0).
    #[validate(range(min = 0.0, message = "Stream rating must be non-negative"))]
    pub stream: Option<f64>,

    /// Jumpstream difficulty rating.
    /// Must be a non-negative decimal value (≥ 0).
    #[validate(range(min = 0.0, message = "Jumpstream rating must be non-negative"))]
    pub jumpstream: Option<f64>,

    /// Handstream difficulty rating.
    /// Must be a non-negative decimal value (≥ 0).
    #[validate(range(min = 0.0, message = "Handstream rating must be non-negative"))]
    pub handstream: Option<f64>,

    /// Stamina difficulty rating.
    /// Must be a non-negative decimal value (≥ 0).
    #[validate(range(min = 0.0, message = "Stamina rating must be non-negative"))]
    pub stamina: Option<f64>,

    /// Jackspeed difficulty rating.
    /// Must be a non-negative decimal value (≥ 0).
    #[validate(range(min = 0.0, message = "Jackspeed rating must be non-negative"))]
    pub jackspeed: Option<f64>,

    /// Chordjack difficulty rating.
    /// Must be a non-negative decimal value (≥ 0).
    #[validate(range(min = 0.0, message = "Chordjack rating must be non-negative"))]
    pub chordjack: Option<f64>,

    /// Technical difficulty rating.
    /// Must be a non-negative decimal value (≥ 0).
    #[validate(range(min = 0.0, message = "Technical rating must be non-negative"))]
    pub technical: Option<f64>,

    /// Timestamp when the score mania rating was created.
    pub created_at: Option<NaiveDateTime>,
}

