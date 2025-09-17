use bigdecimal::BigDecimal;
use validator::ValidationError;

pub(super) fn validate_osu_id(osu_id: i32) -> Result<(), ValidationError> {
    if osu_id < 0 {
        return Err(ValidationError::new("osu_id_must_be_positive"));
    }

    Ok(())
}

pub(super) fn validate_beatmapset_id(beatmapset_id: i32) -> Result<(), ValidationError> {
    if beatmapset_id < 0 {
        return Err(ValidationError::new("beatmapset_id_must_be_positive"));
    }
    Ok(())
}

pub(super) fn validate_difficulty_rating(rating: &BigDecimal) -> Result<(), ValidationError> {
    if *rating < BigDecimal::from(0) || *rating > BigDecimal::from(40) {
        return Err(ValidationError::new("difficulty_rating_out_of_range"));
    }
    Ok(())
}

pub(super) fn validate_bpm(bpm: &BigDecimal) -> Result<(), ValidationError> {
    if *bpm <= BigDecimal::from(0) || *bpm > BigDecimal::from(10000) {
        return Err(ValidationError::new("bpm_out_of_range"));
    }
    Ok(())
}

pub(super) fn validate_cs(cs: &BigDecimal) -> Result<(), ValidationError> {
    if *cs < BigDecimal::from(0) || *cs > BigDecimal::from(10) {
        return Err(ValidationError::new("cs_out_of_range"));
    }
    Ok(())
}

pub(super) fn validate_ar(ar: &BigDecimal) -> Result<(), ValidationError> {
    if *ar < BigDecimal::from(0) || *ar > BigDecimal::from(11) {
        return Err(ValidationError::new("ar_out_of_range"));
    }
    Ok(())
}

pub(super) fn validate_od(od: &BigDecimal) -> Result<(), ValidationError> {
    if *od < BigDecimal::from(0) || *od > BigDecimal::from(10) {
        return Err(ValidationError::new("od_out_of_range"));
    }
    Ok(())
}

pub(super) fn validate_hp(hp: &BigDecimal) -> Result<(), ValidationError> {
    if *hp < BigDecimal::from(0) || *hp > BigDecimal::from(10) {
        return Err(ValidationError::new("hp_out_of_range"));
    }
    Ok(())
}

pub(super) fn validate_mode(mode: i32) -> Result<(), ValidationError> {
    if ![0, 1, 2, 3].contains(&mode) {
        return Err(ValidationError::new("invalid_mode"));
    }
    Ok(())
}

pub(super) fn validate_status(status: &str) -> Result<(), ValidationError> {
    let valid_statuses = [
        "graveyard",
        "wip",
        "pending",
        "ranked",
        "approved",
        "qualified",
        "loved",
    ];
    if !valid_statuses.contains(&status) {
        return Err(ValidationError::new("invalid_status"));
    }
    Ok(())
}
