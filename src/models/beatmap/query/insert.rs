use crate::models::beatmap::types::BeatmapRow;
use sqlx::{Error as SqlxError, PgPool};

pub async fn insert(pool: &PgPool, beatmap: BeatmapRow) -> Result<i32, SqlxError> {
    Ok(sqlx::query!(
        r#"
        INSERT INTO beatmap (
            osu_id, beatmapset_id, difficulty, difficulty_rating,
            count_circles, count_sliders, count_spinners, max_combo,
            drain_time, total_time, bpm, cs, ar, od, hp, mode,
            status, file_md5, file_path
        ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19)
        RETURNING id
        "#,
        beatmap.osu_id,
        beatmap.beatmapset_id,
        beatmap.difficulty,
        beatmap.difficulty_rating,
        beatmap.count_circles,
        beatmap.count_sliders,
        beatmap.count_spinners,
        beatmap.max_combo,
        beatmap.drain_time,
        beatmap.total_time,
        beatmap.bpm,
        beatmap.cs,
        beatmap.ar,
        beatmap.od,
        beatmap.hp,
        beatmap.mode,
        beatmap.status,
        beatmap.file_md5,
        beatmap.file_path
    )
    .fetch_one(pool)
    .await?
    .id)
}
