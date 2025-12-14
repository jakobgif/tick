// Jakob Frenzel
// 10/12/25

#[cfg(test)]
mod tests {
    use axum::{body::Bytes, extract::{Path, Query, State}};
    use todo_backend::{data_structs::SearchParams, handlers::{delete_todo, get_todo, list_todos, search_todos, update_todo}};
    use axum::{response::IntoResponse};
    use serde_json::{Value};
    use sqlx::{Executor, sqlite::{SqliteConnectOptions, SqlitePool}};
    use uuid::Uuid;

    async fn setup_test_db() -> SqlitePool {
        //create unique db in memory
        let db_name = format!("file:{}?mode=memory&cache=shared", Uuid::new_v4());
        let options = SqliteConnectOptions::new().filename(&db_name).create_if_missing(true);
        let connection = SqlitePool::connect_with(options).await.unwrap();
        connection
    }

    async fn populate_test_db(connection: SqlitePool){
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

        //add two dummy entries
        //minimum entry
        sqlx::query("
                INSERT INTO todos (
                    title, creation_date
                )
                VALUES (?, ?)
            ")
            .bind("Test1")
            .bind(1)
            .execute(&connection)
            .await
            .unwrap();

        //full entry
        sqlx::query("
                INSERT INTO todos (
                    title, content, done, priority, creation_date, due_date, finish_date
                )
                VALUES (?, ?, ?, ?, ?, ?, ?)
            ")
            .bind("Test2")
            .bind("Hello, World!")
            .bind(true)
            .bind(1)
            .bind(2)
            .bind(4)
            .bind(3)
            .execute(&connection)
            .await
            .unwrap();
    }

    fn assert_db_item1(json: Value) {
        assert_eq!(json["title"], "Test1");
        assert_eq!(json["content"], "");
        assert_eq!(json["done"], false);
        assert_eq!(json["priority"], 0);
        assert_eq!(json["creation_date"], 1);
        assert_eq!(json["due_date"], 0);
        assert_eq!(json["finish_date"], 0);
    }

    fn assert_db_item2(json: Value){
        assert_eq!(json["title"], "Test2");
        assert_eq!(json["content"], "Hello, World!");
        assert_eq!(json["done"], true);
        assert_eq!(json["priority"], 1);
        assert_eq!(json["creation_date"], 2);
        assert_eq!(json["due_date"], 4);
        assert_eq!(json["finish_date"], 3);
    }

    #[tokio::test]
    async fn test_list_todos() {
        let connection = setup_test_db().await;

        //call handler function on empty database
        let mut response = list_todos(State(connection.clone())).await.into_response();
        //convert the response to a json object so we can check specific keys
        let mut body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let mut json: Value = serde_json::from_slice(&body).unwrap();

        //assert the response
        assert_eq!(json["status"], "error");

        //fill database
        populate_test_db(connection.clone()).await;

        response = list_todos(State(connection.clone())).await.into_response();
        body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        json = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["status"], "ok");

        assert_db_item1(json["items"][0].clone());
        assert_db_item2(json["items"][1].clone());
    }

    #[tokio::test]
    async fn test_get_todo() {
        let connection = setup_test_db().await;
        populate_test_db(connection.clone()).await;

        //unknown id
        let mut response = get_todo(State(connection.clone()), Path(100)).await.into_response();
        let mut body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let mut json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["status"], "error");
        assert_eq!(json["message"], "no rows returned by a query that expected to return at least one row");

        //get item id 1
        response = get_todo(State(connection.clone()), Path(1)).await.into_response();
        body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        json = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["status"], "ok");
        assert_db_item1(json["item"].clone());

        //get item id 2
        response = get_todo(State(connection.clone()), Path(2)).await.into_response();
        body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        json = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["status"], "ok");
        assert_db_item2(json["item"].clone());
    }

    #[tokio::test]
    async fn test_update_todo() {
        let connection = setup_test_db().await;
        populate_test_db(connection.clone()).await;

        //malformed json
        let malformed_json = r#"{}"#;
        let mut response = update_todo(State(connection.clone()), Bytes::from(malformed_json)).await.into_response();
        let mut body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let mut json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["status"], "error");
        assert_eq!(json["message"], "Invalid JSON: missing field `id` at line 1 column 2");

        //non existend id
        let item_json = r#"{"content":"updated content","creation_date":0,"done":true,"finish_date":10,"due_date":20,"id":100,"priority":100,"title":"updated title"}"#;
        response = update_todo(State(connection.clone()), Bytes::from(item_json)).await.into_response();
        body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        json = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["status"], "error");
        assert_eq!(json["message"], "Todo with ID 100 does not exist");

        //update item 1
        let item_json = r#"{"content":"updated content","creation_date":0,"done":true,"finish_date":10,"due_date":20,"id":1,"priority":100,"title":"updated title"}"#;
        response = update_todo(State(connection.clone()), Bytes::from(item_json)).await.into_response();
        body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        json = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["status"], "ok");

        //read back item 1
        response = get_todo(State(connection.clone()), Path(1)).await.into_response();
        body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        json = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["status"], "ok");

        assert_eq!(json["item"]["title"], "updated title");
        assert_eq!(json["item"]["content"], "updated content");
        assert_eq!(json["item"]["done"], true);
        assert_eq!(json["item"]["priority"], 100);
        assert_eq!(json["item"]["creation_date"], 1);
        assert_eq!(json["item"]["due_date"], 20);
        assert_eq!(json["item"]["finish_date"], 10);
    }

    #[tokio::test]
    async fn test_search_todos() {
        let connection = setup_test_db().await;
        populate_test_db(connection.clone()).await;

        //query done=true
        let mut params = SearchParams {
            done: Some(true)
        };
        let mut response = search_todos(State(connection.clone()), Query(params)).await.into_response();
        let mut body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let mut json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["status"], "ok");
        assert_eq!(json["items"].as_array().unwrap().len(), 1);
        assert_db_item2(json["items"][0].clone());

        //query nothing
        params = SearchParams {
            done: None
        };
        response = search_todos(State(connection.clone()), Query(params)).await.into_response();
        body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        json = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["status"], "ok");
        assert_eq!(json["items"].as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_delete_todo() {
        let connection = setup_test_db().await;
        populate_test_db(connection.clone()).await;

        //unknown ID
        let mut response = delete_todo(State(connection.clone()), Path(100)).await.into_response();
        let mut body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let mut json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["status"], "error");
        assert_eq!(json["message"], "Todo with ID 100 does not exist");

        //delete ID 1
        response = delete_todo(State(connection.clone()), Path(1)).await.into_response();
        body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        json = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["status"], "ok");

        //read back todos
        response = list_todos(State(connection.clone())).await.into_response();
        body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        json = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["status"], "ok");
        //only one item in database
        assert_eq!(json["items"].as_array().unwrap().len(), 1);
        //remaining item matches item 2
        assert_db_item2(json["items"][0].clone());
    }
}