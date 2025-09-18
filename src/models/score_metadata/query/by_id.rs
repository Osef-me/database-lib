use crate::define_by_id;
use crate::models::score_metadata::types::ScoreMetadataRow;

define_by_id!(find_by_id, "score_metadata", ScoreMetadataRow,
    "id, skin, pause_count, started_at, ended_at, time_paused, score, accuracy, max_combo, perfect, count_300, count_100, count_50, count_miss, count_katu, count_geki, created_at");
