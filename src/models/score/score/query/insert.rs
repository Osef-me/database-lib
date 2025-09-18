use crate::define_insert_returning_id;
use crate::models::score::score::types::ScoreRow;
// no extra imports needed

define_insert_returning_id!(insert, "score", ScoreRow, user_id, rates_id, score_metadata_id, replay_id, hwid, mods, rank, status);

