use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::model::Task;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Cannot send HTTP request")]
    InvalidRequest(#[from] reqwest::Error),

    #[error("Cannot load tasks from JSON")]
    TasksFetch(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
    tasks: Vec<Task>,
}

impl Database {
    pub async fn new(addr: &str) -> Result {
        let addr = format!("{}/tasks", addr.trim_end_matches('/'));
        let client = reqwest::Client::new();

        let result = client.get(addr)
            .send().await?
            .text().await?;

        serde_json::from_str(&result)
            .map_err(Error::from)
    }
}

pub type Result = std::result::Result<Database, Error>;