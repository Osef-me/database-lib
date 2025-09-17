use crate::models::beatmap::validators::validate_difficulty_rating;
use bigdecimal::BigDecimal;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_difficulty_rating_valid_range() {
        assert!(validate_difficulty_rating(&BigDecimal::from(0)).is_ok());
        assert!(validate_difficulty_rating(&BigDecimal::from(1)).is_ok());
        assert!(validate_difficulty_rating(&BigDecimal::from(20)).is_ok());
        assert!(validate_difficulty_rating(&BigDecimal::from(40)).is_ok());
    }

    #[test]
    fn test_validate_difficulty_rating_invalid_negative() {
        let result = validate_difficulty_rating(&BigDecimal::from(-1));
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "difficulty_rating_out_of_range");
    }

    #[test]
    fn test_validate_difficulty_rating_invalid_too_high() {
        let result = validate_difficulty_rating(&BigDecimal::from(41));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_difficulty_rating_invalid_very_high() {
        let result = validate_difficulty_rating(&BigDecimal::from(100));
        assert!(result.is_err());
    }
}
