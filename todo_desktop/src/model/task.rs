use serde::{Deserialize, Serialize};

use crate::model::id::Id;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    #[serde(skip_serializing)]
    pub id: Option<Id>,
    pub title: String,
    pub completed: bool,
}

impl Task {
    pub fn new(title: impl Into<String>) -> Self {
        Task {
            id: None,
            title: title.into(),
            completed: false,
        }
    }

    pub fn toggle(&mut self) {
        self.completed = !self.completed;
    }
}