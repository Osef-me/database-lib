use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use validator::Validate;
use sqlx::FromRow;

use crate::models::score_rating::validators::*;
use crate::utils::RATING_TYPE_REGEX;

#[derive(Debug, Clone, FromRow, Validate)]
pub struct ScoreRating {
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    #[validate(range(min = 1, message = "Score ID must be positive"))]
    pub score_id: i32,

    #[validate(custom(function = "validate_rating"))]
    pub rating: BigDecimal,

    #[validate(length(min = 1, max = 30, message = "Rating type must be between 1 and 30 characters"))]
    #[validate(regex(path = "*RATING_TYPE_REGEX", message = "Invalid rating type"))]
    pub rating_type: String,
    pub created_at: Option<NaiveDateTime>,
}
