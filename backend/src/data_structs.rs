// Jakob Frenzel
// 10/12/25

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Struct that contains all data a todo item consists of.
/// 
/// This struct matches the database fields.
#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct TodoItem {
    /// unique id for the todo item
    pub id: i64,

    /// title of the todo
    pub title: String,

    /// content of the todo item
    pub content: String,

    /// flag to indicate if task done or not done
    pub done: bool,

    /// priority levels
    /// 
    /// - `0` = least important  
    /// - `i16::MAX` = most important
    pub priority: i16,

    /// datetime when item was created
    /// 
    /// - timestamp created on client side
    /// - serialized as epoch seconds
    #[serde(with = "chrono::serde::ts_seconds")]
    pub creation_date: DateTime<Utc>,

    /// datetime when the task should be finished
    /// 
    /// - serialized as epoch seconds
    #[serde(with = "chrono::serde::ts_seconds")]
    pub due_date: DateTime<Utc>,

    /// datetime when the task actually was finished
    /// 
    /// - timestamp created on client side
    /// - serialized as epoch seconds
    #[serde(with = "chrono::serde::ts_seconds")]
    pub finish_date: DateTime<Utc>
}

/// [`list_todos()`]: crate::handlers::list_todos
/// Struct that contains the query parameters the application offers
/// 
/// Use this struct with [`list_todos()`] to customize the results via
/// pagination, sorting, filtering, and search.
#[derive(Debug, Deserialize, Serialize)]
pub struct QueryParams {
    /// Maximum number of todo items to return.
    pub count: Option<i64>,
    /// Number of items to skip before returning results.
    pub offset: Option<i64>,

    /// Field to sort by.
    /// 
    /// See [`SortBy`] for available options.
    pub sort_by: Option<SortBy>,
    /// Sort order: ascending or descending.
    /// 
    /// See [`Order`] for possible values.
    pub order: Option<Order>,

    /// Filter todos by completion status.
    pub done: Option<bool>,

    /// Search string to filter todos by title or content.
    pub search: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SortBy {
    CreationDate,
    DueDate,
    Priority,
    Done,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Order {
    Asc,
    Desc,
}