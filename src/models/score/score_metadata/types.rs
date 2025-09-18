use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use validator::Validate;
#[derive(Debug, Clone, sqlx::FromRow, Validate)]
pub struct ScoreMetadataRow {
    /// Unique identifier for the score metadata record.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    /// Skin used for the play.
    /// Maximum 100 characters.
    #[validate(length(max = 100, message = "Skin name must be 100 characters or less"))]
    pub skin: Option<String>,

    /// Number of times the game was paused.
    /// Must be a non-negative integer (≥ 0).
    #[validate(range(min = 0, message = "Pause count must be non-negative"))]
    pub pause_count: i32,

    /// Timestamp when the play started.
    /// Used for anti-cheat verification.
    pub started_at: NaiveDateTime,

    /// Timestamp when the play ended.
    /// Used for anti-cheat verification.
    pub ended_at: NaiveDateTime,

    /// Total time paused in seconds.
    /// Must be a non-negative integer (≥ 0).
    #[validate(range(min = 0, message = "Time paused must be non-negative"))]
    pub time_paused: i32,

    /// Score achieved.
    /// Must be a non-negative integer (≥ 0).
    #[validate(range(min = 0, message = "Score must be non-negative"))]
    pub score: i32,

    /// Accuracy achieved (as a percentage).
    /// Must be between 0.0 and 100.0.
    pub accuracy: BigDecimal,

    /// Maximum combo achieved.
    /// Must be a non-negative integer (≥ 0).
    #[validate(range(min = 0, message = "Max combo must be non-negative"))]
    pub max_combo: i32,

    /// Whether the play was perfect (no misses).
    pub perfect: bool,

    /// Number of 300 hits.
    /// Must be a non-negative integer (≥ 0).
    #[validate(range(min = 0, message = "Count 300 must be non-negative"))]
    pub count_300: i32,

    /// Number of 100 hits.
    /// Must be a non-negative integer (≥ 0).
    #[validate(range(min = 0, message = "Count 100 must be non-negative"))]
    pub count_100: i32,

    /// Number of 50 hits.
    /// Must be a non-negative integer (≥ 0).
    #[validate(range(min = 0, message = "Count 50 must be non-negative"))]
    pub count_50: i32,

    /// Number of misses.
    /// Must be a non-negative integer (≥ 0).
    #[validate(range(min = 0, message = "Count miss must be non-negative"))]
    pub count_miss: i32,

    /// Number of katu hits (perfect 100s).
    /// Must be a non-negative integer (≥ 0).
    #[validate(range(min = 0, message = "Count katu must be non-negative"))]
    pub count_katu: i32,

    /// Number of geki hits (perfect 300s).
    /// Must be a non-negative integer (≥ 0).
    #[validate(range(min = 0, message = "Count geki must be non-negative"))]
    pub count_geki: i32,

    /// Timestamp when the score metadata was created.
    pub created_at: Option<NaiveDateTime>,
}
