// Jakob Frenzel
// 10/12/25

use serde::{Deserialize, Serialize};

/// Struct to handle whatever search parameters the application offers
/// To be used with search_todos()
#[derive(Deserialize, Serialize)]
pub struct SearchParams {
    /// optional query to only return done/not done tasks
    pub done: Option<bool>,
}