use axum::{routing::get, Json, Router};
use serde::Serialize;

use crate::{models::rule::Rule, routes::AppState};
use uuid::Uuid;

#[derive(Serialize)]
struct RulesListResponse {
    rules: Vec<RuleSummary>,
}

#[derive(Serialize)]
struct RuleSummary {
    pub id: Uuid,
    pub key: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    /// Pour plus tard : nombre d’utilisateurs impactés, taux de complétion, etc.
    pub impacted_users: Option<i64>,
    pub completion_rate: Option<f64>,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(list_rules))
}

/// Liste des règles (stub – renvoie pour l’instant une règle d’exemple).
async fn list_rules(_state: axum::extract::State<AppState>) -> Json<RulesListResponse> {
    let example = RuleSummary {
        id: Uuid::new_v4(),
        key: "enforce_mfa_critical_apps".to_string(),
        name: "MFA sur les applications critiques".to_string(),
        description: "Si l’utilisateur n’a pas de MFA sur une app critique, on génère une tâche."
            .to_string(),
        enabled: true,
        impacted_users: Some(10),
        completion_rate: Some(0.7),
    };

    Json(RulesListResponse {
        rules: vec![example],
    })
}


