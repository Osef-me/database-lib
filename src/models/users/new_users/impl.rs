use super::query::{find_by_discord_id, find_by_token, insert};
use super::NewUsersRow;
use sqlx::{Error as SqlxError, PgPool};
use uuid::Uuid;

impl NewUsersRow {
    pub async fn insert(self, pool: &PgPool) -> Result<i32, SqlxError> {
        insert(pool, self).await
    }

    pub async fn find_by_discord_id(
        pool: &PgPool,
        discord_id: i64,
    ) -> Result<Option<Self>, SqlxError> {
        find_by_discord_id(pool, discord_id).await
    }

    pub async fn find_by_token(pool: &PgPool, token: Uuid) -> Result<Option<Self>, SqlxError> {
        find_by_token(pool, token).await
    }
}
