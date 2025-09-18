use sqlx::{Error as SqlxError, PgPool, Row};

pub async fn insert(pool: &PgPool, hash: &str, replay_path: &str) -> Result<i32, SqlxError> {
    let rec = sqlx::QueryBuilder::new(
        "INSERT INTO replays (hash, replay_available, replay_path, created_at) VALUES (",
    )
    .push_bind(hash)
    .push(", true, ")
    .push_bind(replay_path)
    .push(", NOW()) RETURNING id")
    .build()
    .fetch_one(pool)
    .await?;
    rec.try_get("id")
}
