use crate::define_insert_returning_id;
use crate::models::pending_beatmap::types::PendingBeatmapRow;

define_insert_returning_id!(insert, "pending_beatmap", PendingBeatmapRow, hash, osu_id);
