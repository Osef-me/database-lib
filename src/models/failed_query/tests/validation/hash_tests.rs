use crate::models::failed_query::FailedQuery;
use validator::Validate;

/// Tests for hash validation in the `FailedQuery` model.
///
/// This module contains comprehensive tests that verify the hash field validation
/// rules, including format validation (alphanumeric only) and length constraints.
///
/// # Test Coverage
///
/// - **Valid cases**: Alphanumeric hashes of various lengths
/// - **Invalid format**: Hashes with special characters, spaces, etc.
/// - **Invalid length**: Empty hashes and hashes exceeding 255 characters
/// - **Edge cases**: Single character hashes, maximum length hashes

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_failed_query_hash_validation_valid_alphanumeric() {
        let failed_query = FailedQuery {
            id: 1,
            hash: "abc123".to_string(),
            created_at: None,
        };
        assert!(failed_query.validate().is_ok());

        let failed_query2 = FailedQuery {
            id: 1,
            hash: "ABC123".to_string(),
            created_at: None,
        };
        assert!(failed_query2.validate().is_ok());

        let failed_query3 = FailedQuery {
            id: 1,
            hash: "a1b2c3".to_string(),
            created_at: None,
        };
        assert!(failed_query3.validate().is_ok());
    }

    #[test]
    fn test_failed_query_hash_validation_valid_single_character() {
        let failed_query = FailedQuery {
            id: 1,
            hash: "a".to_string(),
            created_at: None,
        };
        assert!(failed_query.validate().is_ok());

        let failed_query2 = FailedQuery {
            id: 1,
            hash: "1".to_string(),
            created_at: None,
        };
        assert!(failed_query2.validate().is_ok());
    }

    #[test]
    fn test_failed_query_hash_validation_invalid_empty() {
        let failed_query = FailedQuery {
            id: 1,
            hash: "".to_string(),
            created_at: None,
        };
        assert!(failed_query.validate().is_err());
    }

    #[test]
    fn test_failed_query_hash_validation_invalid_too_long() {
        let long_hash = "a".repeat(256);
        let failed_query = FailedQuery {
            id: 1,
            hash: long_hash,
            created_at: None,
        };
        assert!(failed_query.validate().is_err());
    }

    #[test]
    fn test_failed_query_hash_validation_invalid_with_special_characters() {
        let failed_query = FailedQuery {
            id: 1,
            hash: "abc-123".to_string(),
            created_at: None,
        };
        assert!(failed_query.validate().is_err());

        let failed_query2 = FailedQuery {
            id: 1,
            hash: "abc_123".to_string(),
            created_at: None,
        };
        assert!(failed_query2.validate().is_err());

        let failed_query3 = FailedQuery {
            id: 1,
            hash: "abc.123".to_string(),
            created_at: None,
        };
        assert!(failed_query3.validate().is_err());
    }

    #[test]
    fn test_failed_query_hash_validation_invalid_with_spaces() {
        let failed_query = FailedQuery {
            id: 1,
            hash: "abc 123".to_string(),
            created_at: None,
        };
        assert!(failed_query.validate().is_err());

        let failed_query2 = FailedQuery {
            id: 1,
            hash: " abc123".to_string(),
            created_at: None,
        };
        assert!(failed_query2.validate().is_err());

        let failed_query3 = FailedQuery {
            id: 1,
            hash: "abc123 ".to_string(),
            created_at: None,
        };
        assert!(failed_query3.validate().is_err());
    }

    #[test]
    fn test_failed_query_hash_validation_invalid_with_special_chars() {
        let failed_query = FailedQuery {
            id: 1,
            hash: "abc@123".to_string(),
            created_at: None,
        };
        assert!(failed_query.validate().is_err());

        let failed_query2 = FailedQuery {
            id: 1,
            hash: "abc#123".to_string(),
            created_at: None,
        };
        assert!(failed_query2.validate().is_err());
    }
}
