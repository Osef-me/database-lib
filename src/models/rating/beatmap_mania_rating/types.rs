use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use validator::Validate;

#[derive(Debug, Clone, sqlx::FromRow, Validate)]
pub struct BeatmapManiaRatingRow {
    /// Unique identifier for the beatmap mania rating record.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    /// Reference to the beatmap rating record this mania rating applies to.
    /// Optional field, can be None.
    pub rating_id: Option<i32>,

    /// Stream difficulty rating.
    /// Must be a non-negative decimal value (≥ 0).
    pub stream: Option<BigDecimal>,

    /// Jumpstream difficulty rating.
    /// Must be a non-negative decimal value (≥ 0).
    pub jumpstream: Option<BigDecimal>,

    /// Handstream difficulty rating.
    /// Must be a non-negative decimal value (≥ 0).
    pub handstream: Option<BigDecimal>,

    /// Stamina difficulty rating.
    /// Must be a non-negative decimal value (≥ 0).
    pub stamina: Option<BigDecimal>,

    /// Jackspeed difficulty rating.
    /// Must be a non-negative decimal value (≥ 0).
    pub jackspeed: Option<BigDecimal>,

    /// Chordjack difficulty rating.
    /// Must be a non-negative decimal value (≥ 0).
    pub chordjack: Option<BigDecimal>,

    /// Technical difficulty rating.
    /// Must be a non-negative decimal value (≥ 0).
    pub technical: Option<BigDecimal>,

    /// Timestamp when the mania rating was created.
    pub created_at: Option<NaiveDateTime>,

    /// Timestamp when the mania rating was last updated.
    pub updated_at: Option<NaiveDateTime>,
}
