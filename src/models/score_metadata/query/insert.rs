use crate::define_insert_returning_id;
use crate::models::score_metadata::types::ScoreMetadataRow;


define_insert_returning_id!(
    insert,
    "score_metadata",
    ScoreMetadataRow,
    skin,
    pause_count,
    started_at,
    ended_at,
    time_paused,
    score,
    accuracy,
    max_combo,
    perfect,
    count_300,
    count_100,
    count_50,
    count_miss,
    count_katu,
    count_geki
);
