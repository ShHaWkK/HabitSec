use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use uuid::Uuid;

use crate::{models::team::Team, routes::AppState};

#[derive(Serialize)]
struct TeamsListResponse {
    teams: Vec<Team>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_teams))
        .route("/:id", get(get_team))
}

/// Liste des équipes (stub).
async fn list_teams(State(_state): State<AppState>) -> Json<TeamsListResponse> {
    Json(TeamsListResponse { teams: vec![] })
}

/// Détail d’une équipe (stub).
async fn get_team(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> axum::http::StatusCode {
    axum::http::StatusCode::NOT_FOUND
}


