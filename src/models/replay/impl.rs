use super::query::insert::insert;
use super::types::Replay;
use sqlx::PgPool;

impl Replay {
    pub async fn insert(pool: &PgPool, hash: &str, replay_path: &str) -> Result<Self, sqlx::Error> {
        insert(pool, hash, replay_path).await
    }

    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, sqlx::Error> {
        let replay = sqlx::query_as!(
            Replay,
            r#"
            SELECT id, hash, replay_available, replay_path, created_at
            FROM replays
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(replay)
    }
}
