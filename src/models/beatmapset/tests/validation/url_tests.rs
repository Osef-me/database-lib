use crate::models::beatmapset::validators::validate_url;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_url_valid_http() {
        let url = "http://example.com";
        assert!(validate_url(url).is_ok());
    }

    #[test]
    fn test_validate_url_valid_https() {
        let url = "https://example.com";
        assert!(validate_url(url).is_ok());
    }

    #[test]
    fn test_validate_url_valid_with_path() {
        let url = "https://example.com/path/to/resource";
        assert!(validate_url(url).is_ok());
    }

    #[test]
    fn test_validate_url_invalid_empty() {
        let url = "";
        let result = validate_url(url);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "url_cannot_be_empty");
    }

    #[test]
    fn test_validate_url_invalid_too_long() {
        let long_url = format!("https://example.com/{}", "a".repeat(500));
        let result = validate_url(&long_url);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "url_too_long");
    }

    #[test]
    fn test_validate_url_invalid_no_protocol() {
        let url = "example.com";
        let result = validate_url(url);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "url_must_start_with_http");
    }

    #[test]
    fn test_validate_url_invalid_ftp() {
        let url = "ftp://example.com";
        let result = validate_url(url);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.code, "url_must_start_with_http");
    }
}
