use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::AppError,
    models::task::{Task, TaskStatus},
    routes::AppState,
};

#[derive(Serialize)]
struct TasksListResponse {
    tasks: Vec<Task>,
}

#[derive(Deserialize)]
pub struct UpdateTaskStatusRequest {
    pub status: TaskStatus,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/me", get(list_my_tasks))
        .route("/:id/status", patch(update_task_status))
}

/// Renvoie la liste des tâches de l’utilisateur courant à partir de la base PostgreSQL.
///
/// Pour l’instant on utilise un utilisateur de démo fixe inséré via les migrations
/// (auth réelle à venir via OIDC / JWT).
async fn list_my_tasks(State(state): State<AppState>) -> Result<Json<TasksListResponse>, AppError> {
    // TODO: remplacer par l’ID de l’utilisateur authentifié.
    let demo_user_id =
        Uuid::parse_str("00000000-0000-0000-0000-000000000001").expect("UUID constant valide");

    let tasks = sqlx::query_as::<_, Task>(
        r#"
        SELECT
            id,
            user_id,
            task_type,
            title,
            description,
            status,
            points,
            due_date,
            created_at,
            updated_at
        FROM tasks
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(demo_user_id)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(TasksListResponse { tasks }))
}

/// Mise à jour du statut d’une tâche (stub).
async fn update_task_status(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTaskStatusRequest>,
) -> Result<StatusCode, AppError> {
    sqlx::query(
        r#"
        UPDATE tasks
        SET status = $1, updated_at = now()
        WHERE id = $2
        "#,
    )
    .bind(payload.status)
    .bind(id)
    .execute(&state.pool)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}


