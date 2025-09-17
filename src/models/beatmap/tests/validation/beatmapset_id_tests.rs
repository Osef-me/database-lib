use crate::models::beatmap::validators::validate_beatmapset_id;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_beatmapset_id_valid_positive() {
        assert!(validate_beatmapset_id(1).is_ok());
        assert!(validate_beatmapset_id(100).is_ok());
        assert!(validate_beatmapset_id(1000).is_ok());
    }

    #[test]
    fn test_validate_beatmapset_id_valid_zero() {
        assert!(validate_beatmapset_id(0).is_ok());
    }

    #[test]
    fn test_validate_beatmapset_id_invalid_negative() {
        let result = validate_beatmapset_id(-1);
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert_eq!(error.code, "beatmapset_id_must_be_positive");
    }

    #[test]
    fn test_validate_beatmapset_id_invalid_large_negative() {
        let result = validate_beatmapset_id(-1000);
        assert!(result.is_err());
    }
}
