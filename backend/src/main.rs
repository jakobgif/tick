use axum::{Json, Router, response::IntoResponse, routing::get};
use serde_json::json;

use todo_backend::handlers::{list_todos, search_todos, get_todo, update_todo};

//https://docs.rs/axum/latest/axum/#example
#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/todos", get(list_todos))
        .route("/todos/{id}", get(get_todo).put(update_todo))
        .route("/todos/search", get(search_todos));

    // listen globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started successfully at 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}