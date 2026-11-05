use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    errors::ApiError,
    models::todo::{CreateTodo, Todo, UpdateTodo},
    repositories::todo_repository,
    state::AppState,
};

pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), ApiError> {
    let todo = todo_repository::create(&state.db, payload).await?;

    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn list_todos(State(state): State<AppState>) -> Result<Json<Vec<Todo>>, ApiError> {
    let todos = todo_repository::find_all(&state.db).await?;

    Ok(Json(todos))
}

pub async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Todo>, ApiError> {
    let todo = todo_repository::find_by_id(&state.db, id)
        .await?
        .ok_or(ApiError::TodoNotFound)?;

    Ok(Json(todo))
}

pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, ApiError> {
    if payload.title.is_none() && payload.body.is_none() && payload.completed.is_none() {
        return Err(ApiError::InvalidInput(
            "At least one field must be provided".to_string(),
        ));
    }

    let todo = todo_repository::update(&state.db, id, payload)
        .await?
        .ok_or(ApiError::TodoNotFound)?;

    Ok(Json(todo))
}

pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let deleted = todo_repository::delete(&state.db, id).await?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(ApiError::TodoNotFound)
    }
}
