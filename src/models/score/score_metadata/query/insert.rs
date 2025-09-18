use crate::define_insert_returning_id;
use crate::models::score::score_metadata::types::ScoreMetadataRow;
// no extra imports needed

define_insert_returning_id!(insert, "score_metadata", ScoreMetadataRow, total_score, accuracy, max_combo, count_300, count_100, count_50, count_miss, count_geki, count_katu);

