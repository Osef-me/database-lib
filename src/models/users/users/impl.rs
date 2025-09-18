use super::query::{find_by_discord_id, insert};
use super::UsersRow;
use sqlx::{Error as SqlxError, PgPool};

impl UsersRow {
    pub async fn insert(self, pool: &PgPool) -> Result<i32, SqlxError> {
        insert(pool, self).await
    }

    pub async fn find_by_discord_id(
        pool: &PgPool,
        discord_id: i64,
    ) -> Result<Option<Self>, SqlxError> {
        find_by_discord_id(pool, discord_id).await
    }
}
