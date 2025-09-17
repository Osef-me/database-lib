use crate::models::score::validators::validate_rate;

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::{BigDecimal, FromPrimitive};
    #[test]
    fn test_validate_rate_valid_zero() {
        let rate = BigDecimal::from_f64(0.0).unwrap();
        assert!(validate_rate(&rate).is_ok());
    }

    #[test]
    fn test_validate_rate_valid_normal() {
        let rate = BigDecimal::from_f64(1.5).unwrap();
        assert!(validate_rate(&rate).is_ok());
    }

    #[test]
    fn test_validate_rate_valid_max() {
        let rate = BigDecimal::from_f64(10.0).unwrap();
        assert!(validate_rate(&rate).is_ok());
    }

    #[test]
    fn test_validate_rate_invalid_negative() {
        let rate = BigDecimal::from_f64(-1.0).unwrap();
        let result = validate_rate(&rate);
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert_eq!(error.code, "rate_out_of_range");
    }

    #[test]
    fn test_validate_rate_invalid_too_high() {
        let rate = BigDecimal::from_f64(11.0).unwrap();
        let result = validate_rate(&rate);
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert_eq!(error.code, "rate_out_of_range");
    }

    #[test]
    fn test_validate_rate_invalid_very_high() {
        let rate = BigDecimal::from_f64(100.0).unwrap();
        let result = validate_rate(&rate);
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert_eq!(error.code, "rate_out_of_range");
    }
}
