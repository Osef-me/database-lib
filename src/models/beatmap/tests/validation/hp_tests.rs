use bigdecimal::BigDecimal;
use crate::models::beatmap::validators::validate_hp;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_hp_valid_range() {
        assert!(validate_hp(&BigDecimal::from(0)).is_ok());
        assert!(validate_hp(&BigDecimal::from(1)).is_ok());
        assert!(validate_hp(&BigDecimal::from(5)).is_ok());
        assert!(validate_hp(&BigDecimal::from(10)).is_ok());
    }

    #[test]
    fn test_validate_hp_invalid_negative() {
        let result = validate_hp(&BigDecimal::from(-1));
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert_eq!(error.code, "hp_out_of_range");
    }

    #[test]
    fn test_validate_hp_invalid_too_high() {
        let result = validate_hp(&BigDecimal::from(11));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_hp_invalid_very_high() {
        let result = validate_hp(&BigDecimal::from(20));
        assert!(result.is_err());
    }
}
