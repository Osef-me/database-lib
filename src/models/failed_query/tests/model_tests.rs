use crate::models::failed_query::FailedQuery;
use chrono::DateTime;

/// Tests for the `FailedQuery` model structure and basic functionality.
///
/// This module contains tests that verify the basic behavior and functionality
/// of the `FailedQuery` struct, including model creation, field access, and
/// edge case handling.
///
/// # Test Coverage
///
/// - **Model creation**: Valid and invalid model instantiation
/// - **Field access**: Proper field value retrieval and assignment
/// - **Edge cases**: Minimum/maximum values, optional fields
/// - **Timestamp handling**: Proper handling of `created_at` field

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_failed_query_valid_model() {
        let failed_query = FailedQuery {
            id: 1,
            hash: "abc123def456".to_string(),
            created_at: Some(DateTime::from_timestamp(1640995200, 0).unwrap().naive_utc()),
        };

        // Test que le modèle peut être créé sans erreur
        assert_eq!(failed_query.id, 1);
        assert_eq!(failed_query.hash, "abc123def456");
        assert!(failed_query.created_at.is_some());
    }

    #[test]
    fn test_failed_query_without_created_at() {
        let failed_query = FailedQuery {
            id: 1,
            hash: "abc123def456".to_string(),
            created_at: None,
        };

        assert_eq!(failed_query.id, 1);
        assert_eq!(failed_query.hash, "abc123def456");
        assert!(failed_query.created_at.is_none());
    }

    #[test]
    fn test_failed_query_hash_edge_cases() {
        // Test avec hash de longueur minimale
        let failed_query_min = FailedQuery {
            id: 1,
            hash: "a".to_string(),
            created_at: None,
        };
        assert_eq!(failed_query_min.hash, "a");

        // Test avec hash de longueur maximale (255 caractères)
        let long_hash = "a".repeat(255);
        let failed_query_max = FailedQuery {
            id: 1,
            hash: long_hash.clone(),
            created_at: None,
        };
        assert_eq!(failed_query_max.hash, long_hash);
    }

    #[test]
    fn test_failed_query_id_edge_cases() {
        // Test avec ID minimal
        let failed_query_min = FailedQuery {
            id: 1,
            hash: "abc123".to_string(),
            created_at: None,
        };
        assert_eq!(failed_query_min.id, 1);

        // Test avec ID plus grand
        let failed_query_large = FailedQuery {
            id: 999999,
            hash: "abc123".to_string(),
            created_at: None,
        };
        assert_eq!(failed_query_large.id, 999999);
    }
}
