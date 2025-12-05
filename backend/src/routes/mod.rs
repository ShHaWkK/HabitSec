pub mod auth;
pub mod users;
pub mod teams;
pub mod tasks;
pub mod rules;
pub mod scores;
pub mod health;

use crate::{config::AppConfig, db::DbPool};

/// Etat partagé de l’application pour les handlers Axum.
#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub pool: DbPool,
}


