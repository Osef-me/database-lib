use crate::models::msd::validators::validate_main_pattern;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_main_pattern_valid_simple_json() {
        let pattern = r#"{"type": "stream"}"#.to_string();
        assert!(validate_main_pattern(&pattern).is_ok());
    }

    #[test]
    fn test_validate_main_pattern_valid_complex_json() {
        let pattern = r#"{"patterns": ["stream", "jumpstream", "handstream"], "difficulty": 5.5}"#.to_string();
        assert!(validate_main_pattern(&pattern).is_ok());
    }

    #[test]
    fn test_validate_main_pattern_valid_array_json() {
        let pattern = r#"["stream", "jumpstream", "handstream"]"#.to_string();
        assert!(validate_main_pattern(&pattern).is_ok());
    }

    #[test]
    fn test_validate_main_pattern_valid_nested_json() {
        let pattern = r#"{"stream": {"difficulty": 6.0, "notes": 100}, "jumpstream": {"difficulty": 4.5}}"#.to_string();
        assert!(validate_main_pattern(&pattern).is_ok());
    }

    #[test]
    fn test_validate_main_pattern_invalid_empty() {
        let pattern = "".to_string();
        let result = validate_main_pattern(&pattern);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "main_pattern_cannot_be_empty");
    }

    #[test]
    fn test_validate_main_pattern_invalid_too_long() {
        let long_pattern = "a".repeat(1001);
        let pattern = long_pattern;
        let result = validate_main_pattern(&pattern);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "main_pattern_too_long");
    }

    #[test]
    fn test_validate_main_pattern_invalid_not_json() {
        let pattern = "stream+jumpstream+handstream".to_string();
        let result = validate_main_pattern(&pattern);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "main_pattern_invalid_json");
    }

    #[test]
    fn test_validate_main_pattern_invalid_malformed_json() {
        let pattern = r#"{"type": "stream", "difficulty": 5.5"#.to_string(); // Missing closing brace
        let result = validate_main_pattern(&pattern);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "main_pattern_invalid_json");
    }

    #[test]
    fn test_validate_main_pattern_invalid_json_with_trailing_comma() {
        let pattern = r#"{"type": "stream", "difficulty": 5.5,}"#.to_string(); // Trailing comma
        let result = validate_main_pattern(&pattern);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "main_pattern_invalid_json");
    }
}
