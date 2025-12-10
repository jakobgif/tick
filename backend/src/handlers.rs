// Jakob Frenzel
// 10/12/25

use axum::{Json, response::IntoResponse, extract::Query, extract::Path};
use serde_json::json;

use crate::data_structs::{SearchParams};

/// returns all todo items
/// # Examples
/// ```
/// curl -X GET http://localhost:3000/todos
/// ```
pub async fn list_todos() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "items": "todo item"
    }))
}

/// returns a specific todo item by ID
/// # Examples
/// ```
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
/// ```
/// curl -X PUT http://localhost:3000/todos/31
/// ```
// https://docs.rs/axum/latest/axum/extract/struct.Path.html
pub async fn update_todo(Path(id): Path<u64>) -> impl IntoResponse {
    println!("{}", id);
    Json(json!({
        "status": "ok"
    }))
}

/// return todo items with specific database query
/// # Examples
/// ```
/// curl -X GET http://localhost:3000/todos/search?done=false
/// ```
// https://docs.rs/axum/latest/axum/extract/struct.Query.html
pub async fn search_todos(Query(params): Query<SearchParams>) -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "params": params
    }))
}