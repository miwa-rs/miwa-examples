use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub text: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Todo {
    id: Uuid,
    text: String,
    completed: bool,
}

impl Todo {
    pub fn new(id: Uuid, text: String, completed: bool) -> Self {
        Self {
            id,
            text,
            completed,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn text(&self) -> &str {
        self.text.as_ref()
    }

    pub fn completed(&self) -> bool {
        self.completed
    }
}
