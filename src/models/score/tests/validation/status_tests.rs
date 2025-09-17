use crate::models::score::ScoreRow;
use validator::Validate;

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::{BigDecimal, FromPrimitive};

    #[test]
    fn test_status_validation_valid_pending() {
        let score = ScoreRow {
            id: 1,
            user_id: 12345,
            beatmap_id: 67890,
            score_metadata_id: 11111,
            replay_id: Some(22222),
            rate: BigDecimal::from_f64(1.0).unwrap(),
            hwid: Some("abc123".to_string()),
            mods: 0,
            hash: "def456".to_string(),
            rank: "A".to_string(),
            status: "pending".to_string(),
            created_at: None,
        };
        assert!(score.validate().is_ok());
    }

    #[test]
    fn test_status_validation_valid_processing() {
        let score = ScoreRow {
            id: 1,
            user_id: 12345,
            beatmap_id: 67890,
            score_metadata_id: 11111,
            replay_id: Some(22222),
            rate: BigDecimal::from_f64(1.0).unwrap(),
            hwid: Some("abc123".to_string()),
            mods: 0,
            hash: "def456".to_string(),
            rank: "A".to_string(),
            status: "processing".to_string(),
            created_at: None,
        };
        assert!(score.validate().is_ok());
    }

    #[test]
    fn test_status_validation_valid_validated() {
        let score = ScoreRow {
            id: 1,
            user_id: 12345,
            beatmap_id: 67890,
            score_metadata_id: 11111,
            replay_id: Some(22222),
            rate: BigDecimal::from_f64(1.0).unwrap(),
            hwid: Some("abc123".to_string()),
            mods: 0,
            hash: "def456".to_string(),
            rank: "A".to_string(),
            status: "validated".to_string(),
            created_at: None,
        };
        assert!(score.validate().is_ok());
    }

    #[test]
    fn test_status_validation_valid_cheated() {
        let score = ScoreRow {
            id: 1,
            user_id: 12345,
            beatmap_id: 67890,
            score_metadata_id: 11111,
            replay_id: Some(22222),
            rate: BigDecimal::from_f64(1.0).unwrap(),
            hwid: Some("abc123".to_string()),
            mods: 0,
            hash: "def456".to_string(),
            rank: "A".to_string(),
            status: "cheated".to_string(),
            created_at: None,
        };
        assert!(score.validate().is_ok());
    }

    #[test]
    fn test_status_validation_valid_unsubmitted() {
        let score = ScoreRow {
            id: 1,
            user_id: 12345,
            beatmap_id: 67890,
            score_metadata_id: 11111,
            replay_id: Some(22222),
            rate: BigDecimal::from_f64(1.0).unwrap(),
            hwid: Some("abc123".to_string()),
            mods: 0,
            hash: "def456".to_string(),
            rank: "A".to_string(),
            status: "unsubmitted".to_string(),
            created_at: None,
        };
        assert!(score.validate().is_ok());
    }

    #[test]
    fn test_status_validation_invalid_empty() {
        let score = ScoreRow {
            id: 1,
            user_id: 12345,
            beatmap_id: 67890,
            score_metadata_id: 11111,
            replay_id: Some(22222),
            rate: BigDecimal::from_f64(1.0).unwrap(),
            hwid: Some("abc123".to_string()),
            mods: 0,
            hash: "def456".to_string(),
            rank: "A".to_string(),
            status: "".to_string(),
            created_at: None,
        };
        assert!(score.validate().is_err());
    }

    #[test]
    fn test_status_validation_invalid_too_long() {
        let long_status = "a".repeat(21);
        let score = ScoreRow {
            id: 1,
            user_id: 12345,
            beatmap_id: 67890,
            score_metadata_id: 11111,
            replay_id: Some(22222),
            rate: BigDecimal::from_f64(1.0).unwrap(),
            hwid: Some("abc123".to_string()),
            mods: 0,
            hash: "def456".to_string(),
            rank: "A".to_string(),
            status: long_status,
            created_at: None,
        };
        assert!(score.validate().is_err());
    }

    #[test]
    fn test_status_validation_invalid_unknown_status() {
        let score = ScoreRow {
            id: 1,
            user_id: 12345,
            beatmap_id: 67890,
            score_metadata_id: 11111,
            replay_id: Some(22222),
            rate: BigDecimal::from_f64(1.0).unwrap(),
            hwid: Some("abc123".to_string()),
            mods: 0,
            hash: "def456".to_string(),
            rank: "A".to_string(),
            status: "unknown".to_string(),
            created_at: None,
        };
        assert!(score.validate().is_err());
    }
}
