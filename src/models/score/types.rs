use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct Score {
    pub id: i32,
    pub user_id: i64,
    pub beatmap_id: i32,
    pub score_metadata_id: i32,
    pub replay_id: Option<i32>,
    pub rate: BigDecimal,
    pub hwid: Option<String>,
    pub mods: i64,
    pub hash: String,
    pub rank: String,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
}
