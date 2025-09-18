use crate::define_insert_returning_id;
use crate::models::rating::beatmap_rating::types::BeatmapRatingRow;
// no extra imports needed

define_insert_returning_id!(
    insert,
    "beatmap_rating",
    BeatmapRatingRow,
    rates_id,
    rating,
    rating_type
);
