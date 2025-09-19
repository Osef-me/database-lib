use dotenvy::var;

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgres://postgres:postgres@localhost:5432/template_db".to_string(),
            max_connections: 10,
            min_connections: 1,
        }
    }
}

impl DatabaseConfig {
    pub fn load() -> Self {
        DatabaseConfig {
            url: var("DATABASE_URL").unwrap_or_else(|_| Self::default().url),
            max_connections: var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| Self::default().max_connections.to_string())
                .parse()
                .unwrap_or(Self::default().max_connections),
            min_connections: var("DATABASE_MIN_CONNECTIONS")
                .unwrap_or_else(|_| Self::default().min_connections.to_string())
                .parse()
                .unwrap_or(Self::default().min_connections),
        }
    }
}
