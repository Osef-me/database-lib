use crate::define_insert_returning_id;
use crate::models::rating::score_mania_rating::types::ScoreManiaRatingRow;
// no extra imports needed

define_insert_returning_id!(
    insert,
    "score_mania_rating",
    ScoreManiaRatingRow,
    rating_id,
    stream,
    jumpstream,
    handstream,
    stamina,
    jackspeed,
    chordjack,
    technical
);
