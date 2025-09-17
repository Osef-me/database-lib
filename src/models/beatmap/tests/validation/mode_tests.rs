use crate::models::beatmap::validators::validate_mode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_mode_valid_values() {
        assert!(validate_mode(0).is_ok());
        assert!(validate_mode(1).is_ok());
        assert!(validate_mode(2).is_ok());
        assert!(validate_mode(3).is_ok());
    }

    #[test]
    fn test_validate_mode_invalid_negative() {
        let result = validate_mode(-1);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "invalid_mode");
    }

    #[test]
    fn test_validate_mode_invalid_too_high() {
        let result = validate_mode(4);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_mode_invalid_very_high() {
        let result = validate_mode(10);
        assert!(result.is_err());
    }
}
