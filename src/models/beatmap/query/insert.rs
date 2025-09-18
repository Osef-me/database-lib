use crate::define_insert_returning_id;
use crate::models::beatmap::types::BeatmapRow;

define_insert_returning_id!(
    insert,
    "beatmap",
    BeatmapRow,
    osu_id,
    beatmapset_id,
    difficulty,
    difficulty_rating,
    count_circles,
    count_sliders,
    count_spinners,
    max_combo,
    drain_time,
    total_time,
    bpm,
    cs,
    ar,
    od,
    hp,
    mode,
    status,
    file_md5,
    file_path
);
