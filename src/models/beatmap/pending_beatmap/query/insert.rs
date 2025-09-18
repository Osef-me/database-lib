use crate::define_insert_returning_id;
use crate::models::beatmap::pending_beatmap::types::PendingBeatmapRow;
// no extra imports needed

define_insert_returning_id!(insert, "pending_beatmap", PendingBeatmapRow, osu_hash, osu_id);

