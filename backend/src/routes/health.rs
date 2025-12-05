use axum::{extract::State, Json};
use serde::Serialize;

use crate::routes::AppState;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

/// Endpoint simple de healthcheck.
pub async fn health_check(State(_state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}


