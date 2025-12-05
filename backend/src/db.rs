use sqlx::{Pool, Postgres};

use crate::error::AppError;

/// Alias pratique pour le pool SQLx PostgreSQL.
pub type DbPool = Pool<Postgres>;

/// Initialise le pool de connexion SQLx.
pub async fn init_pool(database_url: &str) -> Result<DbPool, AppError> {
    let pool = sqlx::PgPool::connect(database_url).await?;
    // Optionnel : exécuter un "SELECT 1" pour vérifier la santé de la connexion.
    Ok(pool)
}


