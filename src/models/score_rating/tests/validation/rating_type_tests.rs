use crate::models::score_rating::ScoreRating;
use validator::Validate;

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::BigDecimal;

    #[test]
    fn test_rating_type_validation_valid_etterna() {
        let score_rating = ScoreRating {
            id: 1,
            score_id: 12345,
            rating: BigDecimal::from(85),
            rating_type: "etterna".to_string(),
            created_at: None,
        };
        assert!(score_rating.validate().is_ok());
    }

    #[test]
    fn test_rating_type_validation_valid_osu() {
        let score_rating = ScoreRating {
            id: 1,
            score_id: 12345,
            rating: BigDecimal::from(85),
            rating_type: "osu".to_string(),
            created_at: None,
        };
        assert!(score_rating.validate().is_ok());
    }

    #[test]
    fn test_rating_type_validation_valid_quaver() {
        let score_rating = ScoreRating {
            id: 1,
            score_id: 12345,
            rating: BigDecimal::from(85),
            rating_type: "quaver".to_string(),
            created_at: None,
        };
        assert!(score_rating.validate().is_ok());
    }

    #[test]
    fn test_rating_type_validation_valid_malody() {
        let score_rating = ScoreRating {
            id: 1,
            score_id: 12345,
            rating: BigDecimal::from(85),
            rating_type: "malody".to_string(),
            created_at: None,
        };
        assert!(score_rating.validate().is_ok());
    }

    #[test]
    fn test_rating_type_validation_invalid_empty() {
        let score_rating = ScoreRating {
            id: 1,
            score_id: 12345,
            rating: BigDecimal::from(85),
            rating_type: "".to_string(),
            created_at: None,
        };
        assert!(score_rating.validate().is_err());
    }

    #[test]
    fn test_rating_type_validation_invalid_too_long() {
        let long_type = "a".repeat(31);
        let score_rating = ScoreRating {
            id: 1,
            score_id: 12345,
            rating: BigDecimal::from(85),
            rating_type: long_type,
            created_at: None,
        };
        assert!(score_rating.validate().is_err());
    }

    #[test]
    fn test_rating_type_validation_invalid_unknown_type() {
        let score_rating = ScoreRating {
            id: 1,
            score_id: 12345,
            rating: BigDecimal::from(85),
            rating_type: "unknown".to_string(),
            created_at: None,
        };
        assert!(score_rating.validate().is_err());
    }

    #[test]
    fn test_rating_type_validation_invalid_with_numbers() {
        let score_rating = ScoreRating {
            id: 1,
            score_id: 12345,
            rating: BigDecimal::from(85),
            rating_type: "etterna2".to_string(),
            created_at: None,
        };
        assert!(score_rating.validate().is_err());
    }

    #[test]
    fn test_score_rating_id_validation_invalid_negative() {
        let score_rating = ScoreRating {
            id: -1,
            score_id: 12345,
            rating: BigDecimal::from(85),
            rating_type: "etterna".to_string(),
            created_at: None,
        };
        assert!(score_rating.validate().is_err());
    }

    #[test]
    fn test_score_rating_score_id_validation_invalid_negative() {
        let score_rating = ScoreRating {
            id: 1,
            score_id: -1,
            rating: BigDecimal::from(85),
            rating_type: "etterna".to_string(),
            created_at: None,
        };
        assert!(score_rating.validate().is_err());
    }
}
