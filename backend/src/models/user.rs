use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Source d’identité : Azure AD, Google Workspace, Okta, comptes locaux, etc.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "user_source", rename_all = "lowercase")]
pub enum UserSource {
    AzureAd,
    GoogleWorkspace,
    Okta,
    Local,
}

/// Rôle fonctionnel dans la plateforme.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Employee,
    Admin,
}

/// Utilisateur final de la plateforme.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub department_id: Option<Uuid>,
    pub role: UserRole,
    pub source: UserSource,
    /// Score de conformité humaine (0–100).
    pub compliance_score: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


