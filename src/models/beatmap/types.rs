use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use validator::Validate;

use super::validators::{
    validate_ar, validate_bpm, validate_cs, validate_difficulty_rating, validate_hp, validate_mode,
    validate_od, validate_status,
};

#[derive(Clone, Debug, sqlx::FromRow, Validate)]
pub struct BeatmapRow {
    #[validate(range(min = 1))]
    pub id: i32,
    #[validate(range(min = 0, message = "ID must be positive"))]
    pub osu_id: Option<i32>,
    #[validate(range(min = 0, message = "ID must be positive"))]
    pub beatmapset_id: Option<i32>,
    #[validate(length(min = 1, max = 100))]
    pub difficulty: String,
    #[validate(custom(function = "validate_difficulty_rating"))]
    pub difficulty_rating: BigDecimal,
    #[validate(range(min = 0))]
    pub count_circles: i32,
    #[validate(range(min = 0))]
    pub count_sliders: i32,
    #[validate(range(min = 0))]
    pub count_spinners: i32,
    #[validate(range(min = 1))]
    pub max_combo: i32,
    #[validate(range(min = 0))]
    pub drain_time: i32,
    #[validate(range(min = 0))]
    pub total_time: i32,
    #[validate(custom(function = "validate_bpm"))]
    pub bpm: BigDecimal,
    #[validate(custom(function = "validate_cs"))]
    pub cs: BigDecimal,
    #[validate(custom(function = "validate_ar"))]
    pub ar: BigDecimal,
    #[validate(custom(function = "validate_od"))]
    pub od: BigDecimal,
    #[validate(custom(function = "validate_hp"))]
    pub hp: BigDecimal,
    #[validate(custom(function = "validate_mode"))]
    pub mode: i32,
    #[validate(custom(function = "validate_status"))]
    pub status: String,
    #[validate(length(min = 32, max = 32))]
    pub file_md5: String,
    #[validate(length(min = 1, max = 500))]
    pub file_path: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
