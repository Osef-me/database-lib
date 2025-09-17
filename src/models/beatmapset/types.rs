use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::beatmapset::validators::*;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct BeatmapsetRow {
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    #[validate(range(min = 1, message = "ID must be positive"))]
    pub osu_id: Option<i32>,

    #[validate(length(
        min = 1,
        max = 255,
        message = "Artist must be between 1 and 255 characters"
    ))]
    pub artist: String,

    #[validate(length(max = 255, message = "Artist unicode must be at most 255 characters"))]
    pub artist_unicode: Option<String>,

    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    pub title: String,

    #[validate(length(max = 255, message = "Title unicode must be at most 255 characters"))]
    pub title_unicode: Option<String>,

    #[validate(length(
        min = 1,
        max = 255,
        message = "Creator must be between 1 and 255 characters"
    ))]
    pub creator: String,

    #[validate(length(max = 255, message = "Source must be at most 255 characters"))]
    pub source: Option<String>,

    #[validate(custom(function = "validate_tags"))]
    pub tags: Option<Vec<String>>,

    pub has_video: bool,
    pub has_storyboard: bool,
    pub is_explicit: bool,
    pub is_featured: bool,

    #[validate(custom(function = "validate_url"))]
    pub cover_url: Option<String>,

    #[validate(custom(function = "validate_url"))]
    pub preview_url: Option<String>,

    #[validate(custom(function = "validate_url"))]
    pub osu_file_url: Option<String>,

    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
