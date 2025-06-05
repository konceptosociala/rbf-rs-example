use std::{fmt::Display, str::FromStr, sync::Arc};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

use crate::model::{id::Record, task::Task};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
    pub tasks: Vec<Task>,
}

#[derive(Debug, Clone)]
pub struct DatabaseContext {
    addr: Arc<str>,
    client: reqwest::Client,
}

impl DatabaseContext {
    pub fn new(addr: &str, mode: DatabaseMode) -> DatabaseContext {
        let addr = format!("{mode}://{}", addr
            .trim_end_matches('/')
            .trim_start_matches("http://")
            .trim_start_matches("https://")
        );

        let client = reqwest::Client::new();

        DatabaseContext {
            addr: Arc::from(addr),
            client,
        }
    }

    pub async fn get_tasks(&self) -> Result<Database> {
        let addr = format!("{}/tasks", self.addr);

        let result = self.client.get(addr)
            .send().await?
            .text().await?;

        serde_json::from_str(&result)
            .map_err(Error::from)
    } 

    pub async fn add_task(&self, mut task: Task) -> Result<Task> {
        let addr = format!("{}/tasks", self.addr);

        let result = self.client.post(addr)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&task)?)
            .send().await?
            .text().await?;


        let record: Record = serde_json::from_str(&result)
            .map_err(Error::from)?;

        task.id = Some(record.id);

        Ok(task)
    }

    pub async fn update_task(&self, task: Task) -> Result<Task> {
        let id = task.id.as_ref()
            .ok_or_else(|| Error::TaskIdNotAssigned(task.title.clone()))?;

        let addr = format!("{}/tasks/update/{}", self.addr, id.inner());

        let result = self.client.post(addr)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&task)?)
            .send().await?
            .text().await?;

        let value: Value = serde_json::from_str(&result)
            .map_err(Error::from)?;

        if let Some(Value::Bool(true)) = value.get("updated") {
            Ok(task)
        } else {
            Err(Error::TaskNotFound(task.title.clone()))
        }
    }

    pub async fn delete_task(&self, task: Task) -> Result<Task> {
        let id = task.id.as_ref()
            .ok_or_else(|| Error::TaskIdNotAssigned(task.title.clone()))?;

        let addr = format!("{}/tasks/delete/{}", self.addr, id.inner());

        let result = self.client.post(addr)
            .header("accept", "application/json")
            .send().await?
            .text().await?;

        let value: Value = serde_json::from_str(&result)
            .map_err(Error::from)?;

        if let Some(Value::Bool(true)) = value.get("deleted") {
            Ok(task)
        } else {
            Err(Error::TaskNotFound(task.title.clone()))
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Cannot send HTTP request")]
    InvalidRequest(#[from] reqwest::Error),

    #[error("Cannot load tasks from JSON")]
    TasksFetch(#[from] serde_json::Error),

    #[error("Invalid database mode: `{0}`; expected `http` or `https`")]
    InvalidDatabaseMode(String),

    #[error("Task not found: `{0}`")]
    TaskNotFound(String),

    #[error("Task id is not assigned to the task `{0}`")]
    TaskIdNotAssigned(String),
}

#[derive(Debug, Clone, Copy)]
pub enum DatabaseMode {
    Http,
    Https,
}

impl Display for DatabaseMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseMode::Http => write!(f, "http"),
            DatabaseMode::Https => write!(f, "https"),
        }
    }
}

impl FromStr for DatabaseMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "http" => Ok(DatabaseMode::Http),
            "https" => Ok(DatabaseMode::Https),
            _ => Err(Error::InvalidDatabaseMode(s.to_string())),
        }
    }
}