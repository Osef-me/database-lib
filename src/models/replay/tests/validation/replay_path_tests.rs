use crate::models::replay::Replay;
use validator::Validate;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replay_path_validation_valid_normal() {
        let replay = Replay {
            id: 1,
            hash: "abc123".to_string(),
            replay_available: true,
            replay_path: "/path/to/replay.osr".to_string(),
            created_at: None,
        };
        assert!(replay.validate().is_ok());
    }

    #[test]
    fn test_replay_path_validation_valid_short() {
        let replay = Replay {
            id: 1,
            hash: "abc123".to_string(),
            replay_available: true,
            replay_path: "a.osr".to_string(),
            created_at: None,
        };
        assert!(replay.validate().is_ok());
    }

    #[test]
    fn test_replay_path_validation_valid_long() {
        let long_path = "/very/long/path/to/replay/file/".repeat(10) + "replay.osr";
        let replay = Replay {
            id: 1,
            hash: "abc123".to_string(),
            replay_available: true,
            replay_path: long_path,
            created_at: None,
        };
        assert!(replay.validate().is_ok());
    }

    #[test]
    fn test_replay_path_validation_invalid_empty() {
        let replay = Replay {
            id: 1,
            hash: "abc123".to_string(),
            replay_available: true,
            replay_path: "".to_string(),
            created_at: None,
        };
        assert!(replay.validate().is_err());
    }

    #[test]
    fn test_replay_path_validation_invalid_too_long() {
        let too_long_path = "a".repeat(501);
        let replay = Replay {
            id: 1,
            hash: "abc123".to_string(),
            replay_available: true,
            replay_path: too_long_path,
            created_at: None,
        };
        assert!(replay.validate().is_err());
    }
}
