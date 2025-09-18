use crate::define_insert_returning_id;
use crate::models::scores::score_metadata::types::ScoreMetadataRow;
// no extra imports needed

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
    count_geki,
    count_katu
);
