use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use sqlx::FromRow;
use validator::Validate;

use crate::models::score_metadata::validators::*;

#[derive(Debug, Clone, FromRow, Validate)]
pub struct ScoreMetadataRow {
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    #[validate(length(max = 255, message = "Skin name must be at most 255 characters"))]
    pub skin: Option<String>,

    #[validate(range(min = 0, message = "Pause count cannot be negative"))]
    pub pause_count: i32,

    pub started_at: NaiveDateTime,

    pub ended_at: NaiveDateTime,

    #[validate(range(min = 0, message = "Time paused cannot be negative"))]
    pub time_paused: i32,

    #[validate(range(min = 0, message = "Score cannot be negative"))]
    pub score: i32,

    #[validate(custom(function = "validate_accuracy"))]
    pub accuracy: BigDecimal,

    #[validate(range(min = 0, message = "Max combo cannot be negative"))]
    pub max_combo: i32,

    pub perfect: bool,

    #[validate(range(min = 0, message = "Count 300 cannot be negative"))]
    pub count_300: i32,

    #[validate(range(min = 0, message = "Count 100 cannot be negative"))]
    pub count_100: i32,

    #[validate(range(min = 0, message = "Count 50 cannot be negative"))]
    pub count_50: i32,

    #[validate(range(min = 0, message = "Count miss cannot be negative"))]
    pub count_miss: i32,

    #[validate(range(min = 0, message = "Count katu cannot be negative"))]
    pub count_katu: i32,

    #[validate(range(min = 0, message = "Count geki cannot be negative"))]
    pub count_geki: i32,

    pub created_at: Option<NaiveDateTime>,
}
