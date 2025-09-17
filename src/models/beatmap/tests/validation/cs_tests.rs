use crate::models::beatmap::validators::validate_cs;
use bigdecimal::BigDecimal;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_cs_valid_range() {
        assert!(validate_cs(&BigDecimal::from(0)).is_ok());
        assert!(validate_cs(&BigDecimal::from(1)).is_ok());
        assert!(validate_cs(&BigDecimal::from(5)).is_ok());
        assert!(validate_cs(&BigDecimal::from(10)).is_ok());
    }

    #[test]
    fn test_validate_cs_invalid_negative() {
        let result = validate_cs(&BigDecimal::from(-1));
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "cs_out_of_range");
    }

    #[test]
    fn test_validate_cs_invalid_too_high() {
        let result = validate_cs(&BigDecimal::from(11));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_cs_invalid_very_high() {
        let result = validate_cs(&BigDecimal::from(20));
        assert!(result.is_err());
    }
}
