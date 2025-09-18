use chrono::NaiveDateTime;
use serde_json::Value;
use validator::Validate;

#[derive(Debug, Clone, sqlx::FromRow, Validate)]
pub struct BeatmapRow {
    /// Unique identifier for the beatmap record.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    /// Osu beatmap ID from the official osu! API.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "Osu ID must be positive"))]
    pub osu_id: i32,

    /// Reference to the beatmapset this beatmap belongs to.
    /// Optional field, can be None.
    pub beatmapset_id: Option<i32>,

    /// Difficulty name of the beatmap.
    /// Must be between 1 and 255 characters.
    #[validate(length(
        min = 1,
        max = 255,
        message = "Difficulty must be between 1 and 255 characters"
    ))]
    pub difficulty: String,

    /// Number of circles in the beatmap.
    /// Must be a non-negative integer (≥ 0).
    #[validate(range(min = 0, message = "Count circles must be non-negative"))]
    pub count_circles: i32,

    /// Number of sliders in the beatmap.
    /// Must be a non-negative integer (≥ 0).
    #[validate(range(min = 0, message = "Count sliders must be non-negative"))]
    pub count_sliders: i32,

    /// Number of spinners in the beatmap.
    /// Must be a non-negative integer (≥ 0).
    #[validate(range(min = 0, message = "Count spinners must be non-negative"))]
    pub count_spinners: i32,

    /// Maximum combo possible in the beatmap.
    /// Must be a non-negative integer (≥ 0).
    #[validate(range(min = 0, message = "Max combo must be non-negative"))]
    pub max_combo: i32,

    /// Main pattern data stored as JSON.
    /// Must not be null.
    pub main_pattern: Value,

    /// Circle Size (CS) value.
    /// Must be between 0.0 and 10.0.
    #[validate(range(min = 0.0, max = 10.0, message = "CS must be between 0.0 and 10.0"))]
    pub cs: f64,

    /// Approach Rate (AR) value.
    /// Must be between 0.0 and 10.0.
    #[validate(range(min = 0.0, max = 10.0, message = "AR must be between 0.0 and 10.0"))]
    pub ar: f64,

    /// Overall Difficulty (OD) value.
    /// Must be between 0.0 and 10.0.
    #[validate(range(min = 0.0, max = 10.0, message = "OD must be between 0.0 and 10.0"))]
    pub od: f64,

    /// HP Drain (HP) value.
    /// Must be between 0.0 and 10.0.
    #[validate(range(min = 0.0, max = 10.0, message = "HP must be between 0.0 and 10.0"))]
    pub hp: f64,

    /// Game mode (0=osu!, 1=Taiko, 2=Catch, 3=Mania).
    /// Must be between 0 and 3.
    #[validate(range(min = 0, max = 3, message = "Mode must be between 0 and 3"))]
    pub mode: i32,

    /// Status of the beatmap.
    /// Must be one of: 'pending', 'ranked', 'qualified', 'loved', 'graveyard'.
    #[validate(custom = "validate_status")]
    pub status: String,

    /// Timestamp when the beatmap was created.
    pub created_at: Option<NaiveDateTime>,

    /// Timestamp when the beatmap was last updated.
    pub updated_at: Option<NaiveDateTime>,
}

fn validate_status(status: &str) -> Result<(), validator::ValidationError> {
    match status {
        "pending" | "ranked" | "qualified" | "loved" | "graveyard" => Ok(()),
        _ => Err(validator::ValidationError::new("invalid_status")),
    }
}
