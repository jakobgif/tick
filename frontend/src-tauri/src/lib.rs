use tick_backend::data_structs::{ TodoItem, QueryParams };
use serde_json::Value;
use serde::Deserialize;
use chrono::{DateTime, Utc, TimeZone};

#[derive(Deserialize)]
struct ApiResponse<T> {
    status: String,
    items: Option<T>,
    item: Option<T>,
    message: Option<String>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            fetch_todos,
            toggle_todo_status,
            create_todo,
            update_todo,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn fetch_todos(params: QueryParams) -> Result<Vec<TodoItem>, String> {
    let client = reqwest::Client::new();
    let url = "http://localhost:3000/todos";

    let response = client
        .get(url)
        .query(&params)
        .send()
        .await
        .map_err(|e| { format!("Request error: {}", e) })?;

    //get raw response body
    let raw_body = response.text().await.map_err(|e| { e.to_string() })?;

    //parse JSON from reponse body
    let parsed: ApiResponse<Vec<TodoItem>> = serde_json::from_str(&raw_body).map_err(|e| {format!("JSON parse error: {}", e) })?;

    match parsed.status.as_str() {
        "ok" => {
            let items = parsed.items.unwrap_or_default();
            Ok(items)
        }
        "error" => {
            let msg = parsed.message.unwrap_or("Unknown error".into());
            Err(msg)
        }
        other => {
            Err(format!("Unexpected status: {}", other))
        }
    }
}

#[tauri::command]
async fn toggle_todo_status(id: i64) -> Result<String, String> {
    let client = reqwest::Client::new();

    //get the todo based on id
    let url = format!("http://localhost:3000/todos/{}", id);

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| { format!("Request error: {}", e) })?;

    let raw_body = response.text().await.map_err(|e| { e.to_string() })?;

    let parsed: ApiResponse<TodoItem> = serde_json::from_str(&raw_body).map_err(|e| {format!("JSON parse error: {}", e) })?;

    let mut todo: TodoItem = match parsed.status.as_str() {
        "ok" => {
            parsed.item.ok_or_else(|| "Item not valid".to_string())?
        }
        "error" => {
            return Err(parsed.message.unwrap_or("Unknown error".into()));
        }
        other => {
            return Err(format!("Unexpected status: {}", other));
        }
    };

    //toggle todo item
    todo.done = !todo.done;

    //set finish date
    if todo.done {
        todo.finish_date = Utc::now()
    } else {
        todo.finish_date = Utc.timestamp_opt(0, 0).unwrap();
    }

    //update todo item
    let response = client
        .put(&url)
        .json(&todo)
        .send()
        .await
        .map_err(|e| { format!("Request error: {}", e) })?;

    let raw_body = response.text().await.map_err(|e| { e.to_string() })?;

    let parsed: ApiResponse<TodoItem> = serde_json::from_str(&raw_body).map_err(|e| {format!("JSON parse error: {}", e) })?;

    match parsed.status.as_str() {
        "ok" => {
            Ok("Todo status updated".to_string())
        }
        "error" => {
            let msg = parsed.message.unwrap_or("Unknown error".into());
            Err(msg)
        }
        other => {
            Err(format!("Unexpected status: {}", other))
        }
    }
}

#[tauri::command]
async fn create_todo(mut todo: TodoItem) -> Result<String, String> {
    let client = reqwest::Client::new();

    //url to post a new todo
    let url = format!("http://localhost:3000/todos");

    //set creation date
    todo.creation_date = Utc::now();

    let response = client
        .post(&url)
        .json(&todo)
        .send()
        .await
        .map_err(|e| { format!("Request error: {}", e) })?;

    let raw_body = response.text().await.map_err(|e| { e.to_string() })?;

    let parsed: ApiResponse<TodoItem> = serde_json::from_str(&raw_body).map_err(|e| {format!("JSON parse error: {}", e) })?;

    match parsed.status.as_str() {
        "ok" => {
            Ok("Todo created".to_string())
        }
        "error" => {
            let msg = parsed.message.unwrap_or("Unknown error".into());
            Err(msg)
        }
        other => {
            Err(format!("Unexpected status: {}", other))
        }
    }
}

#[tauri::command]
async fn update_todo(mut todo: TodoItem) -> Result<String, String> {
    let client = reqwest::Client::new();

    //get old todo based on id to check if done status changed
    let url = format!("http://localhost:3000/todos/{}", todo.id);

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| { format!("Request error: {}", e) })?;

    let raw_body = response.text().await.map_err(|e| { e.to_string() })?;

    let parsed: ApiResponse<TodoItem> = serde_json::from_str(&raw_body).map_err(|e| {format!("JSON parse error: {}", e) })?;

    let original_todo: TodoItem = match parsed.status.as_str() {
        "ok" => {
            parsed.item.ok_or_else(|| "Item not valid".to_string())?
        }
        "error" => {
            return Err(parsed.message.unwrap_or("Unknown error".into()));
        }
        other => {
            return Err(format!("Unexpected status: {}", other));
        }
    };

    if !original_todo.done && todo.done {
        todo.finish_date = Utc::now();
    }

    if original_todo.done && !todo.done {
        todo.finish_date = Utc.timestamp_opt(0, 0).unwrap();
    }

    let url = format!("http://localhost:3000/todos/{}", todo.id);

    let response = client
        .put(&url)
        .json(&todo)
        .send()
        .await
        .map_err(|e| { format!("Request error: {}", e) })?;

    let raw_body = response.text().await.map_err(|e| { e.to_string() })?;

    let parsed: ApiResponse<TodoItem> = serde_json::from_str(&raw_body).map_err(|e| {format!("JSON parse error: {}", e) })?;

    match parsed.status.as_str() {
        "ok" => {
            Ok("Todo updated".to_string())
        }
        "error" => {
            let msg = parsed.message.unwrap_or("Unknown error".into());
            Err(msg)
        }
        other => {
            Err(format!("Unexpected status: {}", other))
        }
    }
}
