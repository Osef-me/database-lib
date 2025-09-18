use crate::define_insert_returning_id;
use crate::models::weekly::weekly_participants::types::WeeklyParticipantsRow;
// no extra imports needed

define_insert_returning_id!(
    insert,
    "weekly_participants",
    WeeklyParticipantsRow,
    user_id,
    weekly_id,
    op,
    final_rank
);
