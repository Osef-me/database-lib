use bigdecimal::BigDecimal;
use crate::models::beatmap::validators::validate_ar;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_ar_valid_range() {
        assert!(validate_ar(&BigDecimal::from(0)).is_ok());
        assert!(validate_ar(&BigDecimal::from(1)).is_ok());
        assert!(validate_ar(&BigDecimal::from(5)).is_ok());
        assert!(validate_ar(&BigDecimal::from(10)).is_ok());
        assert!(validate_ar(&BigDecimal::from(11)).is_ok());
    }

    #[test]
    fn test_validate_ar_invalid_negative() {
        let result = validate_ar(&BigDecimal::from(-1));
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert_eq!(error.code, "ar_out_of_range");
    }

    #[test]
    fn test_validate_ar_invalid_too_high() {
        let result = validate_ar(&BigDecimal::from(12));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_ar_invalid_very_high() {
        let result = validate_ar(&BigDecimal::from(20));
        assert!(result.is_err());
    }
}
