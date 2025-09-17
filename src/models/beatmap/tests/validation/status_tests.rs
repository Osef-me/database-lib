use crate::models::beatmap::validators::validate_status;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_status_valid_values() {
        assert!(validate_status("graveyard").is_ok());
        assert!(validate_status("wip").is_ok());
        assert!(validate_status("pending").is_ok());
        assert!(validate_status("ranked").is_ok());
        assert!(validate_status("approved").is_ok());
        assert!(validate_status("qualified").is_ok());
        assert!(validate_status("loved").is_ok());
    }

    #[test]
    fn test_validate_status_invalid_empty() {
        let result = validate_status("");
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "invalid_status");
    }

    #[test]
    fn test_validate_status_invalid_random() {
        let result = validate_status("invalid_status");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_status_invalid_case_sensitive() {
        let result = validate_status("RANKED");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_status_invalid_with_spaces() {
        let result = validate_status(" ranked ");
        assert!(result.is_err());
    }
}
