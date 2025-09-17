use sqlx::{Error as SqlxError, PgPool};

pub async fn count(pool: &PgPool) -> Result<i64, SqlxError> {
    Ok(
        sqlx::query_scalar::<_, i64>(r#"SELECT COUNT(*) FROM pending_beatmap"#)
            .fetch_one(pool)
            .await?,
    )
}
