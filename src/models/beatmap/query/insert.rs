use bigdecimal::BigDecimal;
use sqlx::{Error as SqlxError, PgPool};

pub async fn insert(
    pool: &PgPool,
    osu_id: Option<i32>,
    beatmapset_id: Option<i32>,
    difficulty: String,
    difficulty_rating: BigDecimal,
    count_circles: i32,
    count_sliders: i32,
    count_spinners: i32,
    max_combo: i32,
    drain_time: i32,
    total_time: i32,
    bpm: BigDecimal,
    cs: BigDecimal,
    ar: BigDecimal,
    od: BigDecimal,
    hp: BigDecimal,
    mode: i32,
    status: String,
    file_md5: String,
    file_path: String,
) -> Result<i32, SqlxError> {
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
    )
    .fetch_one(pool)
    .await?
    .id)
}
