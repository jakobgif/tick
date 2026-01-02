// Jakob Frenzel
// 10/12/25

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Struct that contains all data a todo item consists of
/// matches the database fields
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
    /// 0 least important
    /// i16::MAX most important
    pub priority: i16,

    /// datetime when item was created
    /// timestamp created on client side
    /// serialized as epoch seconds
    #[serde(with = "chrono::serde::ts_seconds")]
    pub creation_date: DateTime<Utc>,

    /// datetime when the task should be finished
    /// serialized as epoch seconds
    #[serde(with = "chrono::serde::ts_seconds")]
    pub due_date: DateTime<Utc>,

    /// datetime when the task actually was finished
    /// timestamp created on client side
    /// serialized as epoch seconds
    #[serde(with = "chrono::serde::ts_seconds")]
    pub finish_date: DateTime<Utc>
}

/// Struct that contains the query parameters the application offers
/// To be used with list_todos()
#[derive(Debug, Deserialize, Serialize)]
pub struct QueryParams {
    //query
    //pagination
    pub count: Option<i64>,
    pub offset: Option<i64>,
    //sorting
    pub sort_by: Option<SortBy>,
    pub order: Option<Order>,

    //filter
    pub done: Option<bool>,

    //searchstring
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