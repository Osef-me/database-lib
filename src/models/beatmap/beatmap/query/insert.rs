use crate::define_insert_returning_id;
use crate::models::beatmap::beatmap::types::BeatmapRow;
// no extra imports needed

define_insert_returning_id!(insert, "beatmap", BeatmapRow, osu_id, beatmapset_id, difficulty, count_circles, count_sliders, count_spinners, max_combo, main_pattern, cs, ar, od, hp, mode, status);

