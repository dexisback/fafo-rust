use axum::{Json, extract::Path, extract::State, http::StatusCode};
use uuid::Uuid;

use crate::{
    errors::ApiError,
    models::todo::{CreateTodo, Todo},
    state::AppState,
};

pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> (StatusCode, Json<Todo>) {
    let new_id = Uuid::new_v4();

    let todo = Todo {
        id: new_id,
        title: payload.title,
        body: payload.body,
        is_completed: payload.is_completed,
    };

    state.todos.write().await.insert(todo.id, todo.clone());
    (StatusCode::CREATED, Json(todo))
}

//State<AppState>: Axum extrACTS the application state, Json<Vec<Todo>> : cuh JSOn return hota
pub async fn list_todos(State(state): State<AppState>) -> Json<Vec<Todo>> {
    let todos = state.todos.read().await;
    Json(todos.values().cloned().collect())
}

//State(state) -> extracts the shared applicaton state from the request context
//JSon(payload) -> extracts and deserialises the incoming JSON body
//Statuscode and Json, is what we return as response obv

//write().await v/s read().await -> creating a todo modifies the HashMap, while listing only reads it
//clone() is intentional: we return ownrs Todo values rather than references tied to the lock guard

pub async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Todo>, ApiError> {
    let todos = state.todos.read().await;
    let todo = todos.get(&id).cloned().ok_or(ApiError::TodoNotFound)?;
    Ok(Json(todo))
}

pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    let removed = state.todos.write().await.remove(&id);
    if removed.is_some() {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(ApiError::TodoNotFound)
    }
}
