use crate::models::replay::ReplayRow;
use validator::Validate;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replay_hash_validation_valid_alphanumeric() {
        let replay = ReplayRow {
            id: 1,
            hash: "abc123".to_string(),
            replay_available: true,
            replay_path: "/path/to/replay.osr".to_string(),
            created_at: None,
        };
        assert!(replay.validate().is_ok());

        let replay2 = ReplayRow {
            id: 1,
            hash: "ABC123".to_string(),
            replay_available: true,
            replay_path: "/path/to/replay.osr".to_string(),
            created_at: None,
        };
        assert!(replay2.validate().is_ok());
    }

    #[test]
    fn test_replay_hash_validation_valid_single_character() {
        let replay = ReplayRow {
            id: 1,
            hash: "a".to_string(),
            replay_available: true,
            replay_path: "/path/to/replay.osr".to_string(),
            created_at: None,
        };
        assert!(replay.validate().is_ok());
    }

    #[test]
    fn test_replay_hash_validation_invalid_empty() {
        let replay = ReplayRow {
            id: 1,
            hash: "".to_string(),
            replay_available: true,
            replay_path: "/path/to/replay.osr".to_string(),
            created_at: None,
        };
        assert!(replay.validate().is_err());
    }

    #[test]
    fn test_replay_hash_validation_invalid_too_long() {
        let long_hash = "a".repeat(256);
        let replay = ReplayRow {
            id: 1,
            hash: long_hash,
            replay_available: true,
            replay_path: "/path/to/replay.osr".to_string(),
            created_at: None,
        };
        assert!(replay.validate().is_err());
    }

    #[test]
    fn test_replay_hash_validation_invalid_with_special_characters() {
        let replay = ReplayRow {
            id: 1,
            hash: "abc-123".to_string(),
            replay_available: true,
            replay_path: "/path/to/replay.osr".to_string(),
            created_at: None,
        };
        assert!(replay.validate().is_err());
    }

    #[test]
    fn test_replay_id_validation_invalid_negative() {
        let replay = ReplayRow {
            id: -1,
            hash: "abc123".to_string(),
            replay_available: true,
            replay_path: "/path/to/replay.osr".to_string(),
            created_at: None,
        };
        assert!(replay.validate().is_err());
    }
}
