use crate::models::beatmapset::validators::validate_tags;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_tags_valid_single_tag() {
        let tags = vec!["anime".to_string()];
        assert!(validate_tags(&tags).is_ok());
    }

    #[test]
    fn test_validate_tags_valid_multiple_tags() {
        let tags = vec![
            "anime".to_string(),
            "op".to_string(),
            "tv".to_string(),
        ];
        assert!(validate_tags(&tags).is_ok());
    }

    #[test]
    fn test_validate_tags_invalid_empty_tag() {
        let tags = vec!["".to_string()];
        let result = validate_tags(&tags);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "tag_cannot_be_empty");
    }

    #[test]
    fn test_validate_tags_invalid_too_long() {
        let long_tag = "a".repeat(51);
        let tags = vec![long_tag];
        let result = validate_tags(&tags);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "tag_too_long");
    }

    #[test]
    fn test_validate_tags_invalid_with_newline() {
        let tags = vec!["anime\n".to_string()];
        let result = validate_tags(&tags);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "tag_contains_invalid_characters");
    }

    #[test]
    fn test_validate_tags_invalid_with_tab() {
        let tags = vec!["anime\t".to_string()];
        let result = validate_tags(&tags);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "tag_contains_invalid_characters");
    }

    #[test]
    fn test_validate_tags_invalid_with_carriage_return() {
        let tags = vec!["anime\r".to_string()];
        let result = validate_tags(&tags);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "tag_contains_invalid_characters");
    }
}
