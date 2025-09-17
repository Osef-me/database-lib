use super::super::types::Replay;
use sqlx::{Error as SqlxError, PgPool};

pub async fn insert(pool: &PgPool, hash: &str, replay_path: &str) -> Result<Replay, SqlxError> {
    let replay = sqlx::query_as!(
        Replay,
        r#"
        INSERT INTO replays (hash, replay_available, replay_path, created_at)
        VALUES ($1, true, $2, NOW())
        RETURNING id, hash, replay_available, replay_path, created_at
        "#,
        hash,
        replay_path
    )
    .fetch_one(pool)
    .await?;

    Ok(replay)
}
