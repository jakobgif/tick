// Jakob Frenzel
// 10/12/25

use axum::{Json, response::IntoResponse, extract::Query};
use serde_json::json;

use crate::data_structs::{SearchParams};

/// returns all todo items
/// http://localhost:3000/todos
pub async fn list_todos() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "items": "todo item"
    }))
}

/// return todo items with specific database query
/// http://localhost:3000/todos/search?done=true
/// https://docs.rs/axum/latest/axum/extract/struct.Query.html
pub async fn search_todos(Query(params): Query<SearchParams>) -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "params": params
    }))
}