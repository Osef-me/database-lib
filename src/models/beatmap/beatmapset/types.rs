use chrono::NaiveDateTime;
use validator::Validate;

#[derive(Debug, Clone, sqlx::FromRow, Validate)]
pub struct BeatmapsetRow {
    /// Unique identifier for the beatmapset record.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    /// Osu beatmapset ID from the official osu! API.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "Osu ID must be positive"))]
    pub osu_id: Option<i32>,

    /// Artist name of the beatmapset.
    /// Must be between 1 and 255 characters.
    #[validate(length(
        min = 1,
        max = 255,
        message = "Artist must be between 1 and 255 characters"
    ))]
    pub artist: String,

    /// Unicode artist name of the beatmapset.
    /// Optional field, can be None.
    pub artist_unicode: Option<String>,

    /// Title of the beatmapset.
    /// Must be between 1 and 255 characters.
    #[validate(length(
        min = 1,
        max = 255,
        message = "Title must be between 1 and 255 characters"
    ))]
    pub title: String,

    /// Unicode title of the beatmapset.
    /// Optional field, can be None.
    pub title_unicode: Option<String>,

    /// Creator/mapper of the beatmapset.
    /// Must be between 1 and 255 characters.
    #[validate(length(
        min = 1,
        max = 255,
        message = "Creator must be between 1 and 255 characters"
    ))]
    pub creator: String,

    /// Source of the beatmapset (e.g., anime, game, etc.).
    /// Optional field, can be None.
    pub source: Option<String>,

    /// Tags associated with the beatmapset.
    /// Optional field, can be None.
    pub tags: Option<Vec<String>>,

    /// Whether the beatmapset has a video.
    pub has_video: bool,

    /// Whether the beatmapset has a storyboard.
    pub has_storyboard: bool,

    /// Whether the beatmapset contains explicit content.
    pub is_explicit: bool,

    /// Whether the beatmapset is featured.
    pub is_featured: bool,

    /// URL to the cover image.
    /// Optional field, can be None.
    pub cover_url: Option<String>,

    /// URL to the preview audio.
    /// Optional field, can be None.
    pub preview_url: Option<String>,

    /// URL to the osu file.
    /// Optional field, can be None.
    pub osu_file_url: Option<String>,

    /// Timestamp when the beatmapset was created.
    pub created_at: Option<NaiveDateTime>,

    /// Timestamp when the beatmapset was last updated.
    pub updated_at: Option<NaiveDateTime>,
}
