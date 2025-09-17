use sqlx::{Error as SqlxError, PgPool};

pub async fn insert(pool: &PgPool, hash: &str, replay_path: &str) -> Result<i32, SqlxError> {
    Ok(sqlx::query!(
        r#"
        INSERT INTO replays (hash, replay_available, replay_path, created_at)
        VALUES ($1, true, $2, NOW())
        RETURNING id
        "#,
        hash,
        replay_path
    )
    .fetch_one(pool)
    .await?
    .id)
}
