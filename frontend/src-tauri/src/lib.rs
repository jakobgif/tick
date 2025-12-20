use tick_backend::data_structs::{ TodoItem, QueryParams };
use serde_json::Value;
use serde::Deserialize;

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
        .invoke_handler(tauri::generate_handler![fetch_todos])
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

    // Read raw text first (important!)
    let raw_body = response.text().await.map_err(|e| {
        e.to_string()
    })?;

    // Parse JSON from the raw body
    let parsed: ApiResponse<Vec<TodoItem>> = serde_json::from_str(&raw_body)
        .map_err(|e| {format!("JSON parse error: {}", e) })?;

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
