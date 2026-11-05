use sqlx::PgPool;
use uuid::Uuid;

use crate::models::todo::{CreateTodo, Todo, UpdateTodo};

pub async fn create(pool: &PgPool, todo: CreateTodo) -> Result<Todo, sqlx::Error> {
    sqlx::query_as!(
        Todo,
        r#"
        INSERT INTO todos (id, title, body, completed)
        VALUES ($1, $2, $3, $4)
        RETURNING id, title, body, completed
        "#,
        Uuid::new_v4(),
        todo.title,
        todo.body,
        false
    )
    .fetch_one(pool)
    .await
}

pub async fn find_all(pool: &PgPool) -> Result<Vec<Todo>, sqlx::Error> {
    sqlx::query_as!(
        Todo,
        r#"
        SELECT id, title, body, completed
        FROM todos
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Todo>, sqlx::Error> {
    sqlx::query_as!(
        Todo,
        r#"
        SELECT id, title, body, completed
        FROM todos
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    todo: UpdateTodo,
) -> Result<Option<Todo>, sqlx::Error> {
    sqlx::query_as!(
        Todo,
        r#"
        UPDATE todos
        SET
            title = COALESCE($1, title),
            body = COALESCE($2, body),
            completed = COALESCE($3, completed),
            updated_at = NOW()
        WHERE id = $4
        RETURNING id, title, body, completed
        "#,
        todo.title,
        todo.body,
        todo.completed,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM todos WHERE id = $1", id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}
