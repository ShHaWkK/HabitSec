use axum::{routing::get, Json, Router};
use serde::Serialize;

use crate::{models::score::TeamScoreSummary, routes::AppState};

#[derive(Serialize)]
struct OverviewResponse {
    /// Score global moyen de l’organisation.
    global_score: i32,
}

#[derive(Serialize)]
struct TeamsScoresResponse {
    teams: Vec<TeamScoreSummary>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/overview", get(overview))
        .route("/teams", get(teams_scores))
}

/// Score global (stub).
async fn overview(_state: axum::extract::State<AppState>) -> Json<OverviewResponse> {
    Json(OverviewResponse { global_score: 65 })
}

/// Scores par équipe (stub).
async fn teams_scores(_state: axum::extract::State<AppState>) -> Json<TeamsScoresResponse> {
    Json(TeamsScoresResponse { teams: vec![] })
}


