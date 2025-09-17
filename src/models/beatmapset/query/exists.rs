use sqlx::{Error as SqlxError, PgPool};

pub async fn exists_by_osu_id(pool: &PgPool, osu_id: i32) -> Result<Option<bool>, SqlxError> {
    Ok(sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM beatmapset WHERE osu_id = $1) as exists",
        osu_id
    )
    .fetch_one(pool)
    .await?
    .exists)
}
