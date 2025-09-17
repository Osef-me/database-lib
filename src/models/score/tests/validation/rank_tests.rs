use crate::models::score::ScoreRow;
use validator::Validate;

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::{BigDecimal, FromPrimitive};

    #[test]
    fn test_rank_validation_valid_ss() {
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
            rank: "SS".to_string(),
            status: "validated".to_string(),
            created_at: None,
        };
        assert!(score.validate().is_ok());
    }

    #[test]
    fn test_rank_validation_valid_s() {
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
            rank: "S".to_string(),
            status: "validated".to_string(),
            created_at: None,
        };
        assert!(score.validate().is_ok());
    }

    #[test]
    fn test_rank_validation_valid_a() {
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
    fn test_rank_validation_invalid_empty() {
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
            rank: "".to_string(),
            status: "validated".to_string(),
            created_at: None,
        };
        assert!(score.validate().is_err());
    }

    #[test]
    fn test_rank_validation_invalid_too_long() {
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
            rank: "ABC".to_string(),
            status: "validated".to_string(),
            created_at: None,
        };
        assert!(score.validate().is_err());
    }

    #[test]
    fn test_rank_validation_invalid_unknown_rank() {
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
            rank: "X".to_string(),
            status: "validated".to_string(),
            created_at: None,
        };
        assert!(score.validate().is_err());
    }
}
