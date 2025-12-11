// Jakob Frenzel
// 10/12/25

#[cfg(test)]
mod tests {
    use axum::{extract::{Path, State}};
    use todo_backend::handlers::{get_todo, list_todos};
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
                    goal_date INTEGER,
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
                    title, content, done, priority, creation_date, goal_date, finish_date
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
        assert_eq!(json["goal_date"], 0);
        assert_eq!(json["finish_date"], 0);
    }

    fn assert_db_item2(json: Value){
        assert_eq!(json["title"], "Test2");
        assert_eq!(json["content"], "Hello, World!");
        assert_eq!(json["done"], true);
        assert_eq!(json["priority"], 1);
        assert_eq!(json["creation_date"], 2);
        assert_eq!(json["goal_date"], 4);
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
}