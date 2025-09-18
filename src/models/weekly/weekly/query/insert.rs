use crate::define_insert_returning_id;
use crate::models::weekly::weekly::types::WeeklyRow;
// no extra imports needed

define_insert_returning_id!(insert, "weekly", WeeklyRow, name, end_at, start_at);
