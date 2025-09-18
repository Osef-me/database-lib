use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use validator::Validate;

use crate::utils::HASH_REGEX;

#[derive(Debug, Clone, sqlx::FromRow, Validate)]
pub struct RatesRow {
    /// Unique identifier for the rates record.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    /// Reference to the beatmap this rate applies to.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "Beatmap ID must be positive"))]
    pub beatmap_id: i32,

    /// Osu hash of the beatmap.
    /// Must be between 1 and 128 characters.
    #[validate(length(
        min = 1,
        max = 128,
        message = "Osu hash must be between 1 and 128 characters"
    ))]
    #[validate(regex(path = *HASH_REGEX))]
    pub osu_hash: String,

    /// Rate value in centi (e.g., 110 for 1.1x rate).
    /// Must be between 70 and 200.
    #[validate(range(min = 70, max = 200, message = "Centirate must be between 70 and 200"))]
    pub centirate: i32,

    /// Drain time of the beatmap in seconds.
    /// Must be a non-negative integer (≥ 0).
    #[validate(range(min = 0, message = "Drain time must be non-negative"))]
    pub drain_time: i32,

    /// Total time of the beatmap in seconds.
    /// Must be a non-negative integer (≥ 0).
    #[validate(range(min = 0, message = "Total time must be non-negative"))]
    pub total_time: i32,

    /// Beats per minute of the beatmap.
    /// Must be a positive decimal value.
    pub bpm: BigDecimal,

    /// Timestamp when the rate was created.
    pub created_at: Option<NaiveDateTime>,
}
