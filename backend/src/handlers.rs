// Jakob Frenzel
// 10/12/25

use std::collections::HashMap;

use axum::{Json, body::Bytes, extract::{Path, Query, State}, response::IntoResponse};
use log::{debug, error};
use serde_json::json;
use sqlx::sqlite::{SqlitePool, SqliteQueryResult};
use sqlx::Arguments;

use crate::data_structs::{Order, QueryParams, SortBy, TodoItem};

/// returns a specific range of todo items
/// if nothing is specified it returns 25 items
/// filters can be applied via queryparams
/// # Examples
/// ```bash
/// curl -X GET http://localhost:3000/todos
/// curl -X GET http://localhost:3000/todos?count=2&offset=10
/// curl -X GET http://localhost:3000/todos?done=true
/// ```
// https://docs.rs/axum/latest/axum/extract/struct.Query.html
pub async fn list_todos(State(connection): State<SqlitePool>, Query(params): Query<QueryParams>) -> impl IntoResponse {
    debug!("Called handler list_todos() with {:?}", params);

    // pagination
    let count = params.count.unwrap_or(25).clamp(1, 100);
    let offset = params.offset.unwrap_or(0).max(0);

    let sort_column = match params.sort_by.unwrap_or(SortBy::CreationDate) {
        SortBy::CreationDate => "creation_date",
        SortBy::DueDate => "due_date",
        SortBy::Priority => "priority",
        SortBy::Done => "done",
    };

    let sort_order = match params.order.unwrap_or(Order::Desc) {
        Order::Asc => "ASC",
        Order::Desc => "DESC",
    };

    // base query
    let mut arguments = sqlx::sqlite::SqliteArguments::default();
    let mut query = format!("
        SELECT id, title, content, done, priority,
               creation_date, due_date, finish_date
        FROM todos
        WHERE 1 = 1
    ");
    // https://stackoverflow.com/questions/1264681/what-is-the-purpose-of-using-where-1-1-in-sql-statements

    //append queries
    //filtering
    if let Some(done) = params.done {
        query.push_str(" AND done = ? ");
        let _ = arguments.add(done);
    }

    //sorting and finally pagination
    query.push_str(&format!(
        " ORDER BY {} {} LIMIT ? OFFSET ?",
        sort_column, sort_order
    ));
    let _ = arguments.add(count);
    let _ = arguments.add(offset);

    let result: Result<Vec<TodoItem>, sqlx::Error> = sqlx::query_as_with::<_, TodoItem, _>(&query, arguments)
        .fetch_all(&connection)
        .await;

    match result {
        Ok(items) => {
            debug!("Handler result: Ok");
            Json(json!({
                "status": "ok",
                "items": items
            }))
        }
        Err(e) => {
            error!("Handler result: {:?}", e);
            Json(json!({
                "status": "error",
                "message": e.to_string()
            }))
        }
    }
}

/// returns a specific todo item by ID
/// # Examples
/// ```bash
/// curl -X GET http://localhost:3000/todos/42
/// ```
// https://docs.rs/axum/latest/axum/extract/struct.Path.html
pub async fn get_todo(State(connection): State<SqlitePool>, Path(id): Path<i64>) -> impl IntoResponse {
    debug!("Called handler get_todo() with ID {:?}", id);

    // get database row for specific ID
    let result: Result<TodoItem, sqlx::Error> = sqlx::query_as::<_, TodoItem>("
        SELECT  id, title, content, done, priority,
                creation_date, due_date, finish_date
        FROM todos
        WHERE id = ?
    ")
    .bind(id)
    .fetch_one(&connection)
    .await;

    match result {
        Ok(item) => {
            debug!("Handler result: Ok");
            Json(json!({
                "status": "ok",
                "item": item
            }))
        }
        Err(e) => {
            error!("Handler result: {:?}", e);
            Json(json!({
                "status": "error",
                "message": e.to_string()
            }))
        }
    }
}

/// update a specific todo item by ID
/// the request body must contain a complete TodoItem as json
/// creation date and ID are ignored as they cannot be changed
/// # Examples
/// ```bash
/// curl -X PUT http://localhost:3000/todos/10 -d '{"content":"","creation_date":0,"done":true,"finish_date":0,"due_date":0,"id":0,"priority":0,"title":""}'
/// ```
pub async fn update_todo(State(connection): State<SqlitePool>, Path(id): Path<i64>, body: Bytes) -> impl IntoResponse {
    debug!("Called handler update_todo() with {:?}", body);
    
    //try to parse request body
    let parsed: Result<TodoItem, serde_json::Error> = serde_json::from_slice(&body);

    //handle errors
    let payload = match parsed {
        Ok(p) => p,
        Err(e) => {
            error!("Invalid JSON: {}", e);
            return Json(json!({
                "status": "error",
                "message": format!("Invalid JSON: {}", e)
            }));
        }
    };

    // run udpate query to database
    // creation_date cannot be changed
    let result: Result<SqliteQueryResult, sqlx::Error> = sqlx::query("
        UPDATE todos
        SET title = ?, content = ?, done = ?, priority = ?, due_date = ?, finish_date = ?
        WHERE id = ?
    ")
    .bind(payload.title)
    .bind(payload.content)
    .bind(payload.done)
    .bind(payload.priority)
    .bind(payload.due_date)
    .bind(payload.finish_date)
    .bind(id)
    .execute(&connection)
    .await;

    match result {
        Ok(res) => {
            //ID does not exits = no row got updated
            if res.rows_affected() == 0 {
                error!("Todo with ID {} does not exist", id);
                return Json(json!({
                    "status": "error",
                    "message": format!("Todo with ID {} does not exist", id)
                }));
            }

            debug!("Handler result: Ok");
            Json(json!({ "status": "ok" }))
        }
        Err(e) => {
            error!("Handler result: {:?}", e);
            Json(json!({
                "status": "error",
                "message": e.to_string()
            }))
        }
    }
}

/// return todo items which title or content includes the query string
/// # Examples
/// ```bash
/// curl -X GET http://localhost:3000/todos/autocomplete?q=Hello,%20World!
/// ```
pub async fn autocomplete_todos(State(connection): State<SqlitePool>, Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    debug!("Called handler autocomplete_todos() with {:?}", params);

    //get the query based on the key "q"
    //return empty json if query string is empty
    let query = match params.get("q") {
        Some(q) if !q.trim().is_empty() => q,
        _ => {
            return Json(json!({
                "status": "ok",
                "items": []
            }));
        }
    };

    //prepares string so the sql query "like" works
    //% in front and back means anything can be in front and back
    let query_like = format!("%{}%", query);

    let result = sqlx::query_as::<_, TodoItem>("
        SELECT *
        FROM todos
        WHERE title LIKE ? OR content LIKE ?
        LIMIT 10
    ")
    .bind(&query_like)
    .bind(&query_like)
    .fetch_all(&connection)
    .await;

    match result {
        Ok(items) => {
            debug!("Handler result: Ok");
            Json(json!({
                "status": "ok",
                "items": items
            }))
        }
        Err(e) => {
            error!("Handler result: {:?}", e);
            Json(json!({
                "status": "error",
                "message": e.to_string()
            }))
        }
    }
}

/// delete a specific todo item from the database
/// # Examples
/// ```bash
/// curl -X DELETE http://localhost:3000/todos/42
/// ```
pub async fn delete_todo(State(connection): State<SqlitePool>, Path(id): Path<i64>) -> impl IntoResponse {
    debug!("Called handler delete_todo() with ID {}", id);

    //delete row from database
    let result: Result<SqliteQueryResult, sqlx::Error> = sqlx::query("
        DELETE FROM todos WHERE id = ?
    ")
    .bind(id)
    .execute(&connection)
    .await;

    match result {
        Ok(res) => {
            //ID does not exits = no row got updated
            if res.rows_affected() == 0 {
                error!("Todo with ID {} does not exist", id);
                return Json(json!({
                    "status": "error",
                    "message": format!("Todo with ID {} does not exist", id)
                }));
            }

            debug!("Handler result: Ok");
            Json(json!({ "status": "ok" }))
        }
        Err(e) => {
            error!("Handler result: {:?}", e);
            Json(json!({
                "status": "error",
                "message": e.to_string()
            }))
        }
    }
}