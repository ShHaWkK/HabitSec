use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Historique d’un score de conformité pour un utilisateur.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserScoreSnapshot {
    pub id: Uuid,
    pub user_id: Uuid,
    pub score: i32,
    pub created_at: DateTime<Utc>,
}

/// Agrégat de scores pour une équipe.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamScoreSummary {
    pub team_id: Uuid,
    pub average_score: i32,
    pub users_count: i64,
}


