use crate::models::beatmapset::BeatmapsetRow;
use chrono::DateTime;
use validator::Validate;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beatmapset_valid_model() {
        let beatmapset = BeatmapsetRow {
            id: 1,
            osu_id: Some(12345),
            artist: "Test Artist".to_string(),
            artist_unicode: Some("テストアーティスト".to_string()),
            title: "Test Title".to_string(),
            title_unicode: Some("テストタイトル".to_string()),
            creator: "Test Creator".to_string(),
            source: Some("Test Source".to_string()),
            tags: Some(vec!["anime".to_string(), "op".to_string()]),
            has_video: true,
            has_storyboard: false,
            is_explicit: false,
            is_featured: true,
            cover_url: Some("https://example.com/cover.jpg".to_string()),
            preview_url: Some("https://example.com/preview.mp3".to_string()),
            osu_file_url: Some("https://example.com/beatmap.osz".to_string()),
            created_at: Some(DateTime::from_timestamp(1640995200, 0).unwrap().naive_utc()),
            updated_at: Some(DateTime::from_timestamp(1640995200, 0).unwrap().naive_utc()),
        };

        assert!(beatmapset.validate().is_ok());
        assert_eq!(beatmapset.id, 1);
        assert_eq!(beatmapset.artist, "Test Artist");
        assert_eq!(beatmapset.title, "Test Title");
        assert!(beatmapset.has_video);
        assert!(!beatmapset.has_storyboard);
    }

    #[test]
    fn test_beatmapset_minimal_valid_model() {
        let beatmapset = BeatmapsetRow {
            id: 1,
            osu_id: None,
            artist: "A".to_string(),
            artist_unicode: None,
            title: "T".to_string(),
            title_unicode: None,
            creator: "C".to_string(),
            source: None,
            tags: None,
            has_video: false,
            has_storyboard: false,
            is_explicit: false,
            is_featured: false,
            cover_url: None,
            preview_url: None,
            osu_file_url: None,
            created_at: None,
            updated_at: None,
        };

        assert!(beatmapset.validate().is_ok());
    }

    #[test]
    fn test_beatmapset_invalid_empty_artist() {
        let beatmapset = BeatmapsetRow {
            id: 1,
            osu_id: None,
            artist: "".to_string(),
            artist_unicode: None,
            title: "Test Title".to_string(),
            title_unicode: None,
            creator: "Test Creator".to_string(),
            source: None,
            tags: None,
            has_video: false,
            has_storyboard: false,
            is_explicit: false,
            is_featured: false,
            cover_url: None,
            preview_url: None,
            osu_file_url: None,
            created_at: None,
            updated_at: None,
        };

        assert!(beatmapset.validate().is_err());
    }

    #[test]
    fn test_beatmapset_invalid_empty_title() {
        let beatmapset = BeatmapsetRow {
            id: 1,
            osu_id: None,
            artist: "Test Artist".to_string(),
            artist_unicode: None,
            title: "".to_string(),
            title_unicode: None,
            creator: "Test Creator".to_string(),
            source: None,
            tags: None,
            has_video: false,
            has_storyboard: false,
            is_explicit: false,
            is_featured: false,
            cover_url: None,
            preview_url: None,
            osu_file_url: None,
            created_at: None,
            updated_at: None,
        };

        assert!(beatmapset.validate().is_err());
    }

    #[test]
    fn test_beatmapset_invalid_empty_creator() {
        let beatmapset = BeatmapsetRow {
            id: 1,
            osu_id: None,
            artist: "Test Artist".to_string(),
            artist_unicode: None,
            title: "Test Title".to_string(),
            title_unicode: None,
            creator: "".to_string(),
            source: None,
            tags: None,
            has_video: false,
            has_storyboard: false,
            is_explicit: false,
            is_featured: false,
            cover_url: None,
            preview_url: None,
            osu_file_url: None,
            created_at: None,
            updated_at: None,
        };

        assert!(beatmapset.validate().is_err());
    }

    #[test]
    fn test_beatmapset_invalid_negative_osu_id() {
        let beatmapset = BeatmapsetRow {
            id: 1,
            osu_id: Some(-1),
            artist: "Test Artist".to_string(),
            artist_unicode: None,
            title: "Test Title".to_string(),
            title_unicode: None,
            creator: "Test Creator".to_string(),
            source: None,
            tags: None,
            has_video: false,
            has_storyboard: false,
            is_explicit: false,
            is_featured: false,
            cover_url: None,
            preview_url: None,
            osu_file_url: None,
            created_at: None,
            updated_at: None,
        };

        assert!(beatmapset.validate().is_err());
    }

    #[test]
    fn test_beatmapset_invalid_tags() {
        let beatmapset = BeatmapsetRow {
            id: 1,
            osu_id: None,
            artist: "Test Artist".to_string(),
            artist_unicode: None,
            title: "Test Title".to_string(),
            title_unicode: None,
            creator: "Test Creator".to_string(),
            source: None,
            tags: Some(vec!["".to_string()]), // Empty tag
            has_video: false,
            has_storyboard: false,
            is_explicit: false,
            is_featured: false,
            cover_url: None,
            preview_url: None,
            osu_file_url: None,
            created_at: None,
            updated_at: None,
        };

        assert!(beatmapset.validate().is_err());
    }

    #[test]
    fn test_beatmapset_invalid_url() {
        let beatmapset = BeatmapsetRow {
            id: 1,
            osu_id: None,
            artist: "Test Artist".to_string(),
            artist_unicode: None,
            title: "Test Title".to_string(),
            title_unicode: None,
            creator: "Test Creator".to_string(),
            source: None,
            tags: None,
            has_video: false,
            has_storyboard: false,
            is_explicit: false,
            is_featured: false,
            cover_url: Some("invalid-url".to_string()), // Invalid URL
            preview_url: None,
            osu_file_url: None,
            created_at: None,
            updated_at: None,
        };

        assert!(beatmapset.validate().is_err());
    }

    #[test]
    fn test_beatmapset_invalid_osu_id_zero() {
        let beatmapset = BeatmapsetRow {
            id: 1,
            osu_id: Some(0), // Zero is invalid
            artist: "Test Artist".to_string(),
            artist_unicode: None,
            title: "Test Title".to_string(),
            title_unicode: None,
            creator: "Test Creator".to_string(),
            source: None,
            tags: None,
            has_video: false,
            has_storyboard: false,
            is_explicit: false,
            is_featured: false,
            cover_url: None,
            preview_url: None,
            osu_file_url: None,
            created_at: None,
            updated_at: None,
        };

        assert!(beatmapset.validate().is_err());
    }
}
