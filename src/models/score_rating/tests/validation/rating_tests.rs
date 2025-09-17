use bigdecimal::BigDecimal;
use crate::models::score_rating::validators::validate_rating;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_rating_valid_zero() {
        let rating = BigDecimal::from(0);
        assert!(validate_rating(&rating).is_ok());
    }

    #[test]
    fn test_validate_rating_valid_normal() {
        let rating = BigDecimal::from(75);
        assert!(validate_rating(&rating).is_ok());
    }

    #[test]
    fn test_validate_rating_valid_max() {
        let rating = BigDecimal::from(100);
        assert!(validate_rating(&rating).is_ok());
    }

    #[test]
    fn test_validate_rating_invalid_negative() {
        let rating = BigDecimal::from(-1);
        let result = validate_rating(&rating);
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert_eq!(error.code, "rating_out_of_range");
    }

    #[test]
    fn test_validate_rating_invalid_too_high() {
        let rating = BigDecimal::from(101);
        let result = validate_rating(&rating);
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert_eq!(error.code, "rating_out_of_range");
    }

    #[test]
    fn test_validate_rating_invalid_very_high() {
        let rating = BigDecimal::from(150);
        let result = validate_rating(&rating);
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert_eq!(error.code, "rating_out_of_range");
    }
}
