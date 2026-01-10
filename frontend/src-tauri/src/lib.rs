pub mod api_requests;
pub use crate::api_requests::*;

/// Entry point of the Tauri application. 
/// 
/// Generates handlers for the tauri "commands"
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            fetch_todos,
            toggle_todo_status,
            create_todo,
            update_todo,
            delete_todo,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}