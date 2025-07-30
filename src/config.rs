use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub log_level: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        let _ = dotenv();

        let database_url = env::var("DATABASE_URL")?;
        let server_host = env::var("SERVER_HOST")?;
        let server_port = env::var("SERVER_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(8080);
       
        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());

        Ok(AppConfig {
            database_url,
            server_host,
            server_port,
            log_level,
        })
    }
}
