use crate::models::failed_query::FailedQueryRow;
use validator::Validate;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_failed_query_hash_validation_valid_alphanumeric() {
        let failed_query = FailedQueryRow {
            id: 1,
            hash: "abc123".to_string(),
            created_at: None,
        };
        assert!(failed_query.validate().is_ok());

        let failed_query2 = FailedQueryRow {
            id: 1,
            hash: "ABC123".to_string(),
            created_at: None,
        };
        assert!(failed_query2.validate().is_ok());

        let failed_query3 = FailedQueryRow {
            id: 1,
            hash: "a1b2c3".to_string(),
            created_at: None,
        };
        assert!(failed_query3.validate().is_ok());
    }

    #[test]
    fn test_failed_query_hash_validation_valid_single_character() {
        let failed_query = FailedQueryRow {
            id: 1,
            hash: "a".to_string(),
            created_at: None,
        };
        assert!(failed_query.validate().is_ok());

        let failed_query2 = FailedQueryRow {
            id: 1,
            hash: "1".to_string(),
            created_at: None,
        };
        assert!(failed_query2.validate().is_ok());
    }

    #[test]
    fn test_failed_query_hash_validation_invalid_empty() {
        let failed_query = FailedQueryRow {
            id: 1,
            hash: "".to_string(),
            created_at: None,
        };
        assert!(failed_query.validate().is_err());
    }

    #[test]
    fn test_failed_query_hash_validation_invalid_too_long() {
        let long_hash = "a".repeat(256);
        let failed_query = FailedQueryRow {
            id: 1,
            hash: long_hash,
            created_at: None,
        };
        assert!(failed_query.validate().is_err());
    }

    #[test]
    fn test_failed_query_hash_validation_invalid_with_special_characters() {
        let failed_query = FailedQueryRow {
            id: 1,
            hash: "abc-123".to_string(),
            created_at: None,
        };
        assert!(failed_query.validate().is_err());

        let failed_query2 = FailedQueryRow {
            id: 1,
            hash: "abc_123".to_string(),
            created_at: None,
        };
        assert!(failed_query2.validate().is_err());

        let failed_query3 = FailedQueryRow {
            id: 1,
            hash: "abc.123".to_string(),
            created_at: None,
        };
        assert!(failed_query3.validate().is_err());
    }

    #[test]
    fn test_failed_query_hash_validation_invalid_with_spaces() {
        let failed_query = FailedQueryRow {
            id: 1,
            hash: "abc 123".to_string(),
            created_at: None,
        };
        assert!(failed_query.validate().is_err());

        let failed_query2 = FailedQueryRow {
            id: 1,
            hash: " abc123".to_string(),
            created_at: None,
        };
        assert!(failed_query2.validate().is_err());

        let failed_query3 = FailedQueryRow {
            id: 1,
            hash: "abc123 ".to_string(),
            created_at: None,
        };
        assert!(failed_query3.validate().is_err());
    }

    #[test]
    fn test_failed_query_hash_validation_invalid_with_special_chars() {
        let failed_query = FailedQueryRow {
            id: 1,
            hash: "abc@123".to_string(),
            created_at: None,
        };
        assert!(failed_query.validate().is_err());

        let failed_query2 = FailedQueryRow {
            id: 1,
            hash: "abc#123".to_string(),
            created_at: None,
        };
        assert!(failed_query2.validate().is_err());
    }
}
