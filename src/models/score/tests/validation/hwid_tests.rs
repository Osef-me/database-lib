use crate::models::score::ScoreRow;
use validator::Validate;

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::{BigDecimal, FromPrimitive};

    #[test]
    fn test_hwid_validation_valid_alphanumeric() {
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
    fn test_hwid_validation_valid_none() {
        let score = ScoreRow {
            id: 1,
            user_id: 12345,
            beatmap_id: 67890,
            score_metadata_id: 11111,
            replay_id: Some(22222),
            rate: BigDecimal::from_f64(1.0).unwrap(),
            hwid: None,
            mods: 0,
            hash: "def456".to_string(),
            rank: "A".to_string(),
            status: "validated".to_string(),
            created_at: None,
        };
        assert!(score.validate().is_ok());
    }

    #[test]
    fn test_hwid_validation_invalid_too_long() {
        let long_hwid = "a".repeat(256);
        let score = ScoreRow {
            id: 1,
            user_id: 12345,
            beatmap_id: 67890,
            score_metadata_id: 11111,
            replay_id: Some(22222),
            rate: BigDecimal::from_f64(1.0).unwrap(),
            hwid: Some(long_hwid),
            mods: 0,
            hash: "def456".to_string(),
            rank: "A".to_string(),
            status: "validated".to_string(),
            created_at: None,
        };
        assert!(score.validate().is_err());
    }

    #[test]
    fn test_hwid_validation_invalid_with_special_characters() {
        let score = ScoreRow {
            id: 1,
            user_id: 12345,
            beatmap_id: 67890,
            score_metadata_id: 11111,
            replay_id: Some(22222),
            rate: BigDecimal::from_f64(1.0).unwrap(),
            hwid: Some("abc-123".to_string()),
            mods: 0,
            hash: "def456".to_string(),
            rank: "A".to_string(),
            status: "validated".to_string(),
            created_at: None,
        };
        assert!(score.validate().is_err());
    }
}
