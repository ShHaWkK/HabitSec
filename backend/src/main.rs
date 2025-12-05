//! Point d’entrée du backend Security UX Layer.
//!
//! - Initialise la configuration et la connexion PostgreSQL
//! - Monte le routeur Axum
//! - Active les middlewares (logs structurés, CORS, etc.)

mod config;
mod db;
mod error;
mod models;
mod routes;
mod services;

use axum::{Router, routing::get};
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::AppConfig;
use crate::db::DbPool;

#[tokio::main]
async fn main() -> Result<(), error::AppError> {
    // Initialisation du système de logs (JSON, niveau contrôlé par RUST_LOG)
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer().json())
        .init();

    // Chargement de la configuration (fichiers + variables d’environnement)
    let config = AppConfig::from_env()?;
    tracing::info!(?config, "Configuration chargée");

    // Connexion à la base PostgreSQL via SQLx
    let pool = db::init_pool(&config.database.url).await?;

    // Construction du routeur Axum
    let app = build_app(config.clone(), pool.clone());

    let addr = SocketAddr::from((config.server.host.parse::<std::net::IpAddr>()?, config.server.port));
    tracing::info!(%addr, "Backend Security UX Layer en écoute");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(error::AppError::from)
}

fn build_app(config: AppConfig, pool: DbPool) -> Router {
    use routes::*;

    let api = Router::new()
        .nest("/auth", auth::router())
        .nest("/users", users::router())
        .nest("/teams", teams::router())
        .nest("/tasks", tasks::router())
        .nest("/rules", rules::router())
        .nest("/scores", scores::router())
        .route("/health", get(health::health_check))
        .with_state(routes::AppState { config, pool });

    Router::new()
        .nest("/api", api)
        .layer(
            CorsLayer::permissive() // À resserrer en prod
        )
        .layer(TraceLayer::new_for_http())
}


