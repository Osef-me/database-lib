use crate::define_insert_returning_id;
use crate::models::weekly::weekly_maps::types::WeeklyMapsRow;
// no extra imports needed

define_insert_returning_id!(insert, "weekly_maps", WeeklyMapsRow, weekly_pool_id, beatmap_id);

