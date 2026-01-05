// Jakob Frenzel
// 11/12/25

use axum::{Router, routing::get};
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;
use sqlx::{Executor, sqlite::{SqliteConnectOptions, SqlitePool}};

use tick_backend::handlers::{add_todo, autocomplete_todos, delete_todo, get_todo, list_todos, update_todo};

//https://docs.rs/axum/latest/axum/#example
#[tokio::main]
async fn main() {
    //init log
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .with_module_level("tick_backend", LevelFilter::Debug)
        .init()
        .unwrap();

    info!("Application version: {}", env!("CARGO_PKG_VERSION"));

    // connect to database
    // create new file if it does not exist
    // https://medium.com/@mikecode/rust-sqlx-sqlite-8d66dbe5e497
    let option = SqliteConnectOptions::new().filename("data/todos.db").create_if_missing(true);
    let connection = SqlitePool::connect_with(option).await.unwrap();

    //create table in database if it does not exist
    //based on struct TodoItem
    connection.execute("
            CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                content TEXT,
                done INTEGER NOT NULL DEFAULT 0,
                priority INTEGER,
                creation_date INTEGER NOT NULL,
                due_date INTEGER,
                finish_date INTEGER
            );
        ").await.unwrap();

    let app = Router::new()
        .route("/todos", get(list_todos).post(add_todo))
        .route("/todos/{id}", get(get_todo).delete(delete_todo).put(update_todo))
        .route("/todos/autocomplete", get(autocomplete_todos))
        .with_state(connection);

    // listen globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server started successfully at 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}