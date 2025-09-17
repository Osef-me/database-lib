use crate::models::pending_beatmap::PendingBeatmapRow;
use validator::Validate;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pending_beatmap_hash_validation_valid_alphanumeric() {
        let pending_beatmap = PendingBeatmapRow {
            id: 1,
            hash: "abc123".to_string(),
            osu_id: Some(12345),
            created_at: None,
        };
        assert!(pending_beatmap.validate().is_ok());

        let pending_beatmap2 = PendingBeatmapRow {
            id: 1,
            hash: "ABC123".to_string(),
            osu_id: Some(12345),
            created_at: None,
        };
        assert!(pending_beatmap2.validate().is_ok());
    }

    #[test]
    fn test_pending_beatmap_hash_validation_valid_single_character() {
        let pending_beatmap = PendingBeatmapRow {
            id: 1,
            hash: "a".to_string(),
            osu_id: Some(12345),
            created_at: None,
        };
        assert!(pending_beatmap.validate().is_ok());

        let pending_beatmap2 = PendingBeatmapRow {
            id: 1,
            hash: "1".to_string(),
            osu_id: Some(12345),
            created_at: None,
        };
        assert!(pending_beatmap2.validate().is_ok());
    }

    #[test]
    fn test_pending_beatmap_hash_validation_invalid_empty() {
        let pending_beatmap = PendingBeatmapRow {
            id: 1,
            hash: "".to_string(),
            osu_id: Some(12345),
            created_at: None,
        };
        assert!(pending_beatmap.validate().is_err());
    }

    #[test]
    fn test_pending_beatmap_hash_validation_invalid_too_long() {
        let long_hash = "a".repeat(256);
        let pending_beatmap = PendingBeatmapRow {
            id: 1,
            hash: long_hash,
            osu_id: Some(12345),
            created_at: None,
        };
        assert!(pending_beatmap.validate().is_err());
    }

    #[test]
    fn test_pending_beatmap_hash_validation_invalid_with_special_characters() {
        let pending_beatmap = PendingBeatmapRow {
            id: 1,
            hash: "abc-123".to_string(),
            osu_id: Some(12345),
            created_at: None,
        };
        assert!(pending_beatmap.validate().is_err());

        let pending_beatmap2 = PendingBeatmapRow {
            id: 1,
            hash: "abc_123".to_string(),
            osu_id: Some(12345),
            created_at: None,
        };
        assert!(pending_beatmap2.validate().is_err());
    }

    #[test]
    fn test_pending_beatmap_hash_validation_invalid_with_spaces() {
        let pending_beatmap = PendingBeatmapRow {
            id: 1,
            hash: "abc 123".to_string(),
            osu_id: Some(12345),
            created_at: None,
        };
        assert!(pending_beatmap.validate().is_err());
    }

    #[test]
    fn test_pending_beatmap_id_validation_invalid_negative() {
        let pending_beatmap = PendingBeatmapRow {
            id: -1,
            hash: "abc123".to_string(),
            osu_id: Some(12345),
            created_at: None,
        };
        assert!(pending_beatmap.validate().is_err());
    }

    #[test]
    fn test_pending_beatmap_osu_id_validation_invalid_negative() {
        let pending_beatmap = PendingBeatmapRow {
            id: 1,
            hash: "abc123".to_string(),
            osu_id: Some(-1),
            created_at: None,
        };
        assert!(pending_beatmap.validate().is_err());
    }
}
