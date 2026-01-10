//Jakob Frenzel
//05/01/25

use tick_backend::data_structs::{ TodoItem, QueryParams };
use serde::Deserialize;
use chrono::{ Utc, TimeZone };

#[derive(Deserialize)]
struct ApiResponse<T> {
    status: String,
    items: Option<T>,
    item: Option<T>,
    message: Option<String>,
}

#[tauri::command]
pub async fn fetch_todos(params: QueryParams, api_url: String) -> Result<Vec<TodoItem>, String> {
    let client = reqwest::Client::new();
    let url = format!("{}/todos", api_url);

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
pub async fn toggle_todo_status(id: i64, api_url: String) -> Result<String, String> {
    let client = reqwest::Client::new();

    //get the todo based on id
    let url = format!("{}/todos/{}", api_url, id);

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
pub async fn create_todo(mut todo: TodoItem, api_url: String) -> Result<String, String> {
    let client = reqwest::Client::new();

    //url to post a new todo
    let url = format!("{}/todos", api_url);

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
pub async fn update_todo(mut todo: TodoItem, api_url: String) -> Result<String, String> {
    let client = reqwest::Client::new();

    //get old todo based on id to check if done status changed
    let url = format!("{}/todos/{}", api_url, todo.id);

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

#[tauri::command]
pub async fn delete_todo(id: i64, api_url: String) -> Result<String, String> {
    let client = reqwest::Client::new();

    //delete todo item based on ID
    let url = format!("{}/todos/{}", api_url, id);

    let response = client
        .delete(&url)
        .send()
        .await
        .map_err(|e| { format!("Request error: {}", e) })?;

    let raw_body = response.text().await.map_err(|e| { e.to_string() })?;

    let parsed: ApiResponse<TodoItem> = serde_json::from_str(&raw_body).map_err(|e| {format!("JSON parse error: {}", e) })?;

    match parsed.status.as_str() {
        "ok" => {
            Ok("Todo deleted".to_string())
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