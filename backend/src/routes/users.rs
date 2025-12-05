use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use uuid::Uuid;

use crate::{models::user::User, routes::AppState};

#[derive(Serialize)]
struct UsersListResponse {
    users: Vec<User>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users))
        .route("/:id", get(get_user))
}

/// Liste des utilisateurs (stub – renvoie pour l’instant une liste vide).
async fn list_users(State(_state): State<AppState>) -> Json<UsersListResponse> {
    Json(UsersListResponse { users: vec![] })
}

/// Détail d’un utilisateur (stub – renvoie 404 pour l’instant).
async fn get_user(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> axum::http::StatusCode {
    axum::http::StatusCode::NOT_FOUND
}


