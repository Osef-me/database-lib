use super::query::{find_by_token, insert};
use super::DeviceTokensRow;
use sqlx::{Error as SqlxError, PgPool};
use uuid::Uuid;

impl DeviceTokensRow {
    pub async fn insert(self, pool: &PgPool) -> Result<i32, SqlxError> {
        insert(pool, self).await
    }

    pub async fn find_by_token(pool: &PgPool, token: Uuid) -> Result<Option<Self>, SqlxError> {
        find_by_token(pool, token).await
    }
}
