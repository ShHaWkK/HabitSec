use serde::Deserialize;

use crate::error::AppError;

/// Configuration applicative chargée depuis fichiers + variables d’environnement.
#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AuthConfig {
    /// "local" (dev/démo) ou "oidc" (production).
    pub mode: String,
}

impl AppConfig {
    /// Charge la configuration à partir de `config.*` et des variables d’environnement.
    pub fn from_env() -> Result<Self, AppError> {
        let mut settings = config::Config::builder()
            .add_source(config::File::with_name("backend/config").required(false))
            .add_source(config::Environment::default().separator("__"))
            .build()?;

        // Mapping simple de quelques variables d’env courantes
        if let Ok(url) = std::env::var("DATABASE_URL") {
            settings.set_override("database.url", url)?;
        }
        if let Ok(host) = std::env::var("SERVER_HOST") {
            settings.set_override("server.host", host)?;
        }
        if let Ok(port) = std::env::var("SERVER_PORT") {
            settings.set_override("server.port", port.parse::<u16>().unwrap_or(8080))?;
        }

        let cfg: AppConfig = settings.try_deserialize()?;
        Ok(cfg)
    }
}


