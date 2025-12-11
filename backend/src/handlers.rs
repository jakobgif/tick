// Jakob Frenzel
// 10/12/25

use axum::{Json, extract::{Path, Query, State}, response::IntoResponse};
use serde_json::json;
use sqlx::sqlite::SqlitePool;

use crate::data_structs::{SearchParams, TodoItem};

/// returns all todo items
/// # Examples
/// ```bash
/// curl -X GET http://localhost:3000/todos
/// ```
pub async fn list_todos(State(connection): State<SqlitePool>) -> impl IntoResponse {
    // get all database rows as result so we can check later if query was sucessful or not
    let result: Result<Vec<TodoItem>, sqlx::Error> = sqlx::query_as::<_, TodoItem>("
        SELECT  id, title, content, done, priority,
                creation_date, goal_date, finish_date
        FROM todos
    ")
    .fetch_all(&connection)
    .await;

    match result {
        Ok(items) => Json(json!({
            "status": "ok",
            "items": items
        })),

        Err(e) => Json(json!({
            "status": "error",
            "message": e.to_string()
        })),
    }
}

/// returns a specific todo item by ID
/// # Examples
/// ```bash
/// curl -X GET http://localhost:3000/todos/42
/// ```
// https://docs.rs/axum/latest/axum/extract/struct.Path.html
pub async fn get_todo(Path(id): Path<u64>) -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "item": id
    }))
}

/// update a specific todo item by ID
/// # Examples
/// ```bash
/// curl -X PUT http://localhost:3000/todos/31
/// ```
// https://docs.rs/axum/latest/axum/extract/struct.Path.html
pub async fn update_todo(Path(id): Path<u64>) -> impl IntoResponse {
    Json(json!({
        "status": "ok"
    }))
}

/// return todo items with specific database query
/// # Examples
/// ```bash
/// curl -X GET http://localhost:3000/todos/search?done=false
/// ```
// https://docs.rs/axum/latest/axum/extract/struct.Query.html
pub async fn search_todos(Query(params): Query<SearchParams>) -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "params": params
    }))
}