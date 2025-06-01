use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Task {
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResult {
    pub id: Thing,
    pub title: String,
    pub completed: bool,
}