use crate::models::msd::validators::validate_msd_value;
use bigdecimal::BigDecimal;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_msd_value_valid_zero() {
        let value = BigDecimal::from(0);
        assert!(validate_msd_value(&value).is_ok());
    }

    #[test]
    fn test_validate_msd_value_valid_positive() {
        let value = BigDecimal::from(5);
        assert!(validate_msd_value(&value).is_ok());
    }

    #[test]
    fn test_validate_msd_value_valid_max() {
        let value = BigDecimal::from(20);
        assert!(validate_msd_value(&value).is_ok());
    }

    #[test]
    fn test_validate_msd_value_invalid_negative() {
        let value = BigDecimal::from(-1);
        let result = validate_msd_value(&value);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "msd_value_out_of_range");
    }

    #[test]
    fn test_validate_msd_value_invalid_too_high() {
        let value = BigDecimal::from(21);
        let result = validate_msd_value(&value);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "msd_value_out_of_range");
    }

    #[test]
    fn test_validate_msd_value_invalid_very_high() {
        let value = BigDecimal::from(100);
        let result = validate_msd_value(&value);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "msd_value_out_of_range");
    }
}
