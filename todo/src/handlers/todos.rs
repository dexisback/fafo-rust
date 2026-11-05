use axum::{Json, extract::State, http::StatusCode};

// use uuid::Uuid;

use crate::{
    models::todo::{CreateTodo, Todo},
    state::AppState,
};

pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> (StatusCode, Json<Todo>) {
    use uuid::Uuid;

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

pub async fn list_todos(State(state): State<AppState>) -> Json<Vec<Todo>> {
    let todos = state.todos.read().await;
    Json(todos.values().cloned().collect())
}

//State(state) -> extracts the shared applicaton state from the request context
//JSon(payload) -> extracts and deserialises the incoming JSON body
//Statuscode and Json, is what we return as response obv

//write().await v/s read().await -> creating a todo modifies the HashMap, while listing only reads it
//clone() is intentional: we return ownrs Todo values rather than references tied to the lock guard
