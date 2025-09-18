use chrono::NaiveDateTime;
use validator::Validate;

#[derive(Debug, Clone, sqlx::FromRow, Validate)]
pub struct ScoreRow {
    /// Unique identifier for the score record.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "ID must be positive"))]
    pub id: i32,

    /// Discord ID of the user who achieved this score.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "User ID must be positive"))]
    pub user_id: i64,

    /// Reference to the rates record this score applies to.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "Rates ID must be positive"))]
    pub rates_id: i32,

    /// Reference to the score metadata record.
    /// Must be a positive integer (≥ 1).
    #[validate(range(min = 1, message = "Score metadata ID must be positive"))]
    pub score_metadata_id: i32,

    /// Reference to the replay record.
    /// Optional field, can be None.
    pub replay_id: Option<i32>,

    /// Hardware ID of the computer used to play the score.
    /// Optional field, can be None.
    pub hwid: Option<String>,

    /// Mods used in the play.
    /// Must be a non-negative integer (≥ 0).
    #[validate(range(min = 0, message = "Mods must be non-negative"))]
    pub mods: i64,

    /// Rank achieved in the play.
    /// Must be one of: 'XH', 'X', 'SH', 'SS', 'S', 'A', 'B', 'C', 'D', 'E', 'F', 'G'.
    #[validate(custom(function = "validate_rank"))]
    pub rank: String,

    /// Status of the score.
    /// Must be one of: 'pending', 'processing', 'validated', 'cheated', 'unsubmitted'.
    #[validate(custom(function = "validate_status"))]
    pub status: String,

    /// Timestamp when the score was created.
    pub created_at: Option<NaiveDateTime>,
}

fn validate_rank(rank: &str) -> Result<(), validator::ValidationError> {
    match rank {
        "XH" | "X" | "SH" | "SS" | "S" | "A" | "B" | "C" | "D" | "E" | "F" | "G" => Ok(()),
        _ => Err(validator::ValidationError::new("invalid_rank")),
    }
}

fn validate_status(status: &str) -> Result<(), validator::ValidationError> {
    match status {
        "pending" | "processing" | "validated" | "cheated" | "unsubmitted" => Ok(()),
        _ => Err(validator::ValidationError::new("invalid_status")),
    }
}
