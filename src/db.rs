//! # Database Module
//!
//! Ce module gère la connexion et les opérations avec la base de données PostgreSQL.
//! Il utilise SQLx pour les requêtes asynchrones et la gestion du pool de connexions.
//!

use crate::config::DatabaseConfig;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct DatabaseManager {
    pool: Option<PgPool>,
}

impl Default for DatabaseManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DatabaseManager {
    pub fn new() -> Self {
        Self { pool: None }
    }

    pub async fn connect(&mut self, config: &DatabaseConfig) -> Result<(), sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .connect(&config.url)
            .await?;

        self.pool = Some(pool);
        tracing::info!(
            "Connected to database with {} max connections",
            config.max_connections
        );
        Ok(())
    }

    pub fn get_pool(&self) -> &PgPool {
        self.pool.as_ref().expect("Database not initialized")
    }
}
