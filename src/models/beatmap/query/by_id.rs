use crate::define_by_id;
use crate::models::beatmap::types::BeatmapRow;
use sqlx::{Error as SqlxError, PgPool};

define_by_id!(
    find_by_id,
    "beatmap",
    BeatmapRow,
    "id, osu_id, beatmapset_id, difficulty, difficulty_rating, count_circles,
     count_sliders, count_spinners, max_combo, drain_time, total_time, bpm, cs, 
     ar, od, hp, mode, status, file_md5, file_path, created_at"
);

pub async fn find_by_beatmapset_id(pool: &PgPool, id: i32) -> Result<Option<i32>, SqlxError> {
    Ok(
        sqlx::query!("SELECT beatmapset_id FROM beatmap WHERE id = $1", id)
            .fetch_optional(pool)
            .await?
            .and_then(|r| r.beatmapset_id),
    )
}
