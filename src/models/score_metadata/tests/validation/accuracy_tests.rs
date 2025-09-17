use crate::models::score_metadata::validators::validate_accuracy;
use bigdecimal::BigDecimal;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_accuracy_valid_zero() {
        let accuracy = BigDecimal::from(0);
        assert!(validate_accuracy(&accuracy).is_ok());
    }

    #[test]
    fn test_validate_accuracy_valid_normal() {
        let accuracy = BigDecimal::from(85);
        assert!(validate_accuracy(&accuracy).is_ok());
    }

    #[test]
    fn test_validate_accuracy_valid_max() {
        let accuracy = BigDecimal::from(100);
        assert!(validate_accuracy(&accuracy).is_ok());
    }

    #[test]
    fn test_validate_accuracy_invalid_negative() {
        let accuracy = BigDecimal::from(-1);
        let result = validate_accuracy(&accuracy);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "accuracy_out_of_range");
    }

    #[test]
    fn test_validate_accuracy_invalid_too_high() {
        let accuracy = BigDecimal::from(101);
        let result = validate_accuracy(&accuracy);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "accuracy_out_of_range");
    }

    #[test]
    fn test_validate_accuracy_invalid_very_high() {
        let accuracy = BigDecimal::from(150);
        let result = validate_accuracy(&accuracy);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "accuracy_out_of_range");
    }
}
