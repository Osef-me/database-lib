use crate::define_insert_returning_id;
use crate::models::rating::score_rating::types::ScoreRatingRow;
// no extra imports needed

define_insert_returning_id!(insert, "score_rating", ScoreRatingRow, score_id, rating, rating_type);

