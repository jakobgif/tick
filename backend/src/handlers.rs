// Jakob Frenzel
// 10/12/25

use axum::{Json, body::Bytes, extract::{Path, Query, Request, State}, response::IntoResponse};
use serde_json::json;
use sqlx::sqlite::{SqlitePool, SqliteQueryResult};
use urlencoding::decode;
use sqlx::Arguments;

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
                creation_date, due_date, finish_date
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
pub async fn get_todo(State(connection): State<SqlitePool>, Path(id): Path<i64>) -> impl IntoResponse {
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
/// the request body must contain a complete TodoItem as json
/// # Examples
/// ```bash
/// curl -X PUT http://localhost:3000/todos -d '{"content":"","creation_date":0,"done":true,"finish_date":0,"due_date":0,"id":1,"priority":0,"title":""}'
/// ```
pub async fn update_todo(State(connection): State<SqlitePool>, body: Bytes) -> impl IntoResponse {
    //try to parse request body
    let parsed: Result<TodoItem, serde_json::Error> = serde_json::from_slice(&body);

    //handle errors
    let payload = match parsed {
        Ok(p) => p,
        Err(e) => {
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
    .bind(payload.id)
    .execute(&connection)
    .await;

    match result {
        Ok(res) => {
            //ID does not exits = no row got updated
            if res.rows_affected() == 0 {
                return Json(json!({
                    "status": "error",
                    "message": format!("Todo with ID {} does not exist", payload.id)
                }));
            }

            Json(json!({ "status": "ok" }))
        }
        Err(e) => Json(json!({
            "status": "error",
            "message": e.to_string()
        })),
    }
}

/// return todo items with specific database query
/// # Examples
/// ```bash
/// curl -X GET http://localhost:3000/todos/search?done=true
/// ```
// https://docs.rs/axum/latest/axum/extract/struct.Query.html
pub async fn search_todos(State(connection): State<SqlitePool>, Query(params): Query<SearchParams>) -> impl IntoResponse {
    // https://stackoverflow.com/questions/1264681/what-is-the-purpose-of-using-where-1-1-in-sql-statements
    let mut query = String::from("SELECT * FROM todos WHERE 1 = 1");
    let mut arguments = sqlx::sqlite::SqliteArguments::default();

    if let Some(done) = params.done {
        query.push_str(" AND done = ?");
        let _ = arguments.add(done);
    }

    // get all database rows as result so we can check later if query was sucessful or not
    let result: Result<Vec<TodoItem>, sqlx::Error> = sqlx::query_as_with::<_, TodoItem, _>(&query, arguments)
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