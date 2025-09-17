use crate::models::msd::validators::validate_rate_value;
use bigdecimal::{BigDecimal, FromPrimitive};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_rate_value_valid_none() {
        assert!(validate_rate_value(&BigDecimal::from_f64(0.5).unwrap()).is_ok());
    }

    #[test]
    fn test_validate_rate_value_valid_min() {
        let value = BigDecimal::from_f64(0.5).unwrap();
            assert!(validate_rate_value(&value).is_ok());
        }

    #[test]
    fn test_validate_rate_value_valid_normal() {
        let value = BigDecimal::from_f64(1.0).unwrap();
        assert!(validate_rate_value(&value).is_ok());
    }

    #[test]
    fn test_validate_rate_value_valid_max() {
        let value = BigDecimal::from_f64(2.0).unwrap();
        assert!(validate_rate_value(&value).is_ok());
    }

    #[test]
    fn test_validate_rate_value_invalid_too_low() {
        let value = BigDecimal::from_f64(0.4).unwrap();
        let result = validate_rate_value(&value);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "rate_value_out_of_range");
    }

    #[test]
    fn test_validate_rate_value_invalid_too_high() {
        let value = BigDecimal::from_f64(2.1).unwrap();
        let result = validate_rate_value(&value);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "rate_value_out_of_range");
    }

    #[test]
    fn test_validate_rate_value_invalid_negative() {
        let value = BigDecimal::from_f64(-0.5).unwrap();
        let result = validate_rate_value(&value);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "rate_value_out_of_range");
    }
}
