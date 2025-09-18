use crate::define_insert_returning_id;
use crate::models::beatmap::rates::types::RatesRow;
// no extra imports needed

define_insert_returning_id!(insert, "rates", RatesRow, beatmap_id, osu_hash, centirate, drain_time, total_time, bpm);

