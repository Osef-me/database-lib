use crate::define_insert_returning_id;
use crate::models::rating::beatmap_mania_rating::types::BeatmapManiaRatingRow;
// no extra imports needed

define_insert_returning_id!(insert, "beatmap_mania_rating", BeatmapManiaRatingRow, rating_id, stream, jumpstream, handstream, stamina, jackspeed, chordjack, technical);

