use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use todo_api_derive::ApiModel;

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiModel)]
pub struct Task {
    pub title: String,
    pub completed: bool,
}