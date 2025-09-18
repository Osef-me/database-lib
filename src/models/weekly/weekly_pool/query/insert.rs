use crate::define_insert_returning_id;
use crate::models::weekly::weekly_pool::types::WeeklyPoolRow;
// no extra imports needed

define_insert_returning_id!(
    insert,
    "weekly_pool",
    WeeklyPoolRow,
    beatmap_id,
    weekly_id,
    vote_count
);
