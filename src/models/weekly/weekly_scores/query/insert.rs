use crate::define_insert_returning_id;
use crate::models::weekly::weekly_scores::types::WeeklyScoresRow;
// no extra imports needed

define_insert_returning_id!(insert, "weekly_scores", WeeklyScoresRow, weekly_id, score_id);

