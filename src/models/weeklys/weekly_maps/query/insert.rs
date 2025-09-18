use crate::define_insert_returning_id;
use crate::models::weeklys::weekly_maps::types::WeeklyMapsRow;
// no extra imports needed

define_insert_returning_id!(
    insert,
    "weekly_maps",
    WeeklyMapsRow,
    beatmap_id,
    weekly_id,
    max_rate
);
