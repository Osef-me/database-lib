use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use validator::Validate;
use sqlx::FromRow;

use super::validators::*;
use crate::utils::{HASH_REGEX, RANK_REGEX, SCORE_STATUS_REGEX};

#[derive(Debug, Clone, FromRow, Validate)]
pub struct ScoreRow {
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    #[validate(range(min = 1, message = "User ID must be positive"))]
    pub user_id: i64,

    #[validate(range(min = 1, message = "Beatmap ID must be positive"))]
    pub beatmap_id: i32,

    #[validate(range(min = 1, message = "Score metadata ID must be positive"))]
    pub score_metadata_id: i32,

    #[validate(range(min = 1, message = "Replay ID must be positive"))]
    pub replay_id: Option<i32>,

    #[validate(custom(function = "validate_rate"))]
    pub rate: BigDecimal,

    #[validate(length(max = 255, message = "HWID must be at most 255 characters"))]
    #[validate(regex(path = "*HASH_REGEX", message = "HWID must contain only alphanumeric characters"))]
    pub hwid: Option<String>,

    #[validate(range(min = 0, message = "Mods must be positive"))]
    pub mods: i64,

    #[validate(length(min = 1, max = 255, message = "Hash must be between 1 and 255 characters"))]
    #[validate(regex(path = "*HASH_REGEX", message = "Hash must contain only alphanumeric characters"))]
    pub hash: String,

    #[validate(length(min = 1, max = 2, message = "Rank must be between 1 and 2 characters"))]
    #[validate(regex(path = "*RANK_REGEX", message = "Rank must contain only alphanumeric characters"))]
    pub rank: String,

    #[validate(length(min = 1, max = 20, message = "Status must be between 1 and 20 characters"))]
    #[validate(regex(path = "*SCORE_STATUS_REGEX", message = "Status must contain only alphanumeric characters"))]
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
}
