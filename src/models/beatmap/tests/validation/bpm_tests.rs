use bigdecimal::BigDecimal;
use crate::models::beatmap::validators::validate_bpm;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_bpm_valid_range() {
        assert!(validate_bpm(&BigDecimal::from(1)).is_ok());
        assert!(validate_bpm(&BigDecimal::from(120)).is_ok());
        assert!(validate_bpm(&BigDecimal::from(200)).is_ok());
        assert!(validate_bpm(&BigDecimal::from(10000)).is_ok());
    }

    #[test]
    fn test_validate_bpm_invalid_zero() {
        let result = validate_bpm(&BigDecimal::from(0));
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert_eq!(error.code, "bpm_out_of_range");
    }

    #[test]
    fn test_validate_bpm_invalid_negative() {
        let result = validate_bpm(&BigDecimal::from(-1));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_bpm_invalid_too_high() {
        let result = validate_bpm(&BigDecimal::from(10001));
        assert!(result.is_err());
    }
}
