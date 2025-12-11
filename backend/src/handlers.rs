// Jakob Frenzel
// 10/12/25

use axum::{Json, extract::{Path, Query, Request, State}, response::IntoResponse};
use serde_json::json;
use sqlx::sqlite::SqlitePool;
use urlencoding::decode;

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
pub async fn get_todo(State(connection): State<SqlitePool>, Path(id): Path<u64>) -> impl IntoResponse {
    // get database row for specific ID
    let result: Result<TodoItem, sqlx::Error> = sqlx::query_as::<_, TodoItem>("
        SELECT  id, title, content, done, priority,
                creation_date, goal_date, finish_date
        FROM todos
        WHERE id = ?
    ")
    .bind(id as i64)
    .fetch_one(&connection)
    .await;

    match result {
        Ok(item) => Json(json!({
            "status": "ok",
            "item": item
        })),

        Err(e) => Json(json!({
            "status": "error",
            "message": e.to_string()
        })),
    }
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

/// return todo items which title or content includes the query string
/// # Examples
/// ```bash
/// curl -X GET http://localhost:3000/todos/autocomplete?Hello,%20World!
/// ```
pub async fn autocomplete_todos(req: Request) -> impl IntoResponse {
    //get decoded string out of the query
    //if query is empty the string is empty
    let query = decode(req.uri().query().unwrap_or("")).expect("UTF-8");

    //return nothing if string is empty
    if query.is_empty() {
        return Json(json!({
            "status": "ok",
            "items": []
        }));
    }

    Json(json!({
        "status": "ok",
        "query": query
    }))
}