use validator::ValidationError;

/// Validates that tags are properly formatted if provided.
pub fn validate_tags(tags: &Vec<String>) -> Result<(), ValidationError> {
    for tag in tags {
        if tag.is_empty() {
            return Err(ValidationError::new("tag_cannot_be_empty"));
        }
        if tag.len() > 50 {
            return Err(ValidationError::new("tag_too_long"));
        }
        // Check for invalid characters in tags
        if tag
            .chars()
            .any(|c| c.is_control() || c == '\n' || c == '\r' || c == '\t')
        {
            return Err(ValidationError::new("tag_contains_invalid_characters"));
        }
    }

    Ok(())
}

/// Validates that a URL is properly formatted if provided.
pub fn validate_url(url: &str) -> Result<(), ValidationError> {
    if url.is_empty() {
        return Err(ValidationError::new("url_cannot_be_empty"));
    }
    if url.len() > 500 {
        return Err(ValidationError::new("url_too_long"));
    }
    // Basic URL validation
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(ValidationError::new("url_must_start_with_http"));
    }
    Ok(())
}
