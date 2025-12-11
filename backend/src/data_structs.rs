// Jakob Frenzel
// 10/12/25

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Struct that contains all data a todo item consists of
/// matches the database fields
#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct TodoItem {
    /// unique id for the todo item
    pub id: u64,

    /// title of the todo
    pub title: String,

    /// content of the todo item
    pub content: String,

    /// flag to indicate if task done or not done
    pub done: bool,

    /// priority levels
    /// 0 most important
    /// u16::MAX least important
    pub priority: u16,

    /// datetime when item was created
    /// timestamp created on client side
    /// serialized as epoch seconds
    #[serde(with = "chrono::serde::ts_seconds")]
    pub creation_date: DateTime<Utc>,

    /// datetime when the task should be finished
    /// serialized as epoch seconds
    #[serde(with = "chrono::serde::ts_seconds")]
    pub goal_date: DateTime<Utc>,

    /// datetime when the task actually was finished
    /// timestamp created on client side
    /// serialized as epoch seconds
    #[serde(with = "chrono::serde::ts_seconds")]
    pub finish_date: DateTime<Utc>
}

/// Struct to handle whatever search parameters the application offers
/// To be used with search_todos()
#[derive(Deserialize, Serialize)]
pub struct SearchParams {
    /// optional query to only return done/not done tasks
    pub done: Option<bool>,
}