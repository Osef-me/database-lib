use super::super::types::ScoreRow;
use crate::define_insert_returning_row;

define_insert_returning_row!(insert, "score", ScoreRow,
    user_id, beatmap_id, score_metadata_id, replay_id, rate, hwid, mods, hash, rank, status;
    "id, user_id, beatmap_id, score_metadata_id, replay_id, rate, hwid, mods, hash, rank, status, created_at");
