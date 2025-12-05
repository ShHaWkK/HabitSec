use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::routes::AppState;

/// Représente un utilisateur authentifié côté API (vue simplifiée pour le frontend).
#[derive(Debug, Serialize)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
    pub display_name: String,
    pub role: String,
    pub compliance_score: i32,
}

#[derive(Debug, Deserialize)]
pub struct LocalLoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: AuthUser,
}

pub fn router() -> Router<AppState> {
    Router::new()
        // Mode local de dev : faux login par email/mot de passe
        .route("/login", post(local_login))
        // OIDC : endpoints stub pour l’instant
        .route("/oidc/login", get(oidc_login))
        .route("/oidc/callback", get(oidc_callback))
        .route("/me", get(me))
}

/// Endpoint de login local (dev/démo uniquement).
/// Dans un premier temps, retourne un token factice et un user mocké.
async fn local_login(
    Json(payload): Json<LocalLoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    if payload.email.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // TODO: remplacer par une vraie vérification en base.
    let user = AuthUser {
        id: "00000000-0000-0000-0000-000000000000".to_string(),
        email: payload.email,
        display_name: "Demo User".to_string(),
        role: "employee".to_string(),
        compliance_score: 42,
    };

    let resp = LoginResponse {
        token: "demo-token".to_string(),
        user,
    };

    Ok(Json(resp))
}

/// Point d’entrée OIDC (stub) – renverra plus tard une redirection vers le provider.
async fn oidc_login() -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

/// Callback OIDC (stub) – traitera plus tard le code d’auth.
async fn oidc_callback() -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

/// Retourne les informations de l’utilisateur courant (stub).
async fn me() -> Json<AuthUser> {
    // TODO: extraire l’utilisateur depuis le token / session.
    Json(AuthUser {
        id: "00000000-0000-0000-0000-000000000000".to_string(),
        email: "demo@example.com".to_string(),
        display_name: "Demo User".to_string(),
        role: "employee".to_string(),
        compliance_score: 42,
    })
}


