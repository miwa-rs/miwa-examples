use std::{
    collections::HashMap,
    ops::Deref,
    sync::{Arc, RwLock},
};

use miwa::derive::Injectable;
use uuid::Uuid;

use crate::{
    error::TodoError,
    model::{CreateTodo, Pagination, Todo, UpdateTodo},
};

#[async_trait::async_trait]
pub trait TodoRepo {
    async fn create(&self, create: CreateTodo) -> Result<Todo, TodoError>;
    async fn update(&self, id: Uuid, update: UpdateTodo) -> Result<Todo, TodoError>;
    async fn get(&self, id: Uuid) -> Result<Todo, TodoError>;
    async fn fetch(&self, pagination: Pagination) -> Result<Vec<Todo>, TodoError>;
    async fn delete(&self, id: Uuid) -> Result<(), TodoError>;
}

#[derive(Injectable, Clone)]
pub struct TodoRepoImpl(Arc<dyn TodoRepo + Send + Sync + 'static>);

impl Deref for TodoRepoImpl {
    type Target = dyn TodoRepo + Send + Sync + 'static;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl TodoRepoImpl {
    pub fn new(repo: impl TodoRepo + Send + Sync + 'static) -> Self {
        TodoRepoImpl(Arc::new(repo))
    }
}

pub struct InMemoryTodoRepo(Arc<RwLock<HashMap<Uuid, Todo>>>);

impl InMemoryTodoRepo {
    pub fn new() -> InMemoryTodoRepo {
        InMemoryTodoRepo(Arc::new(RwLock::new(HashMap::new())))
    }
}

#[async_trait::async_trait]
impl TodoRepo for InMemoryTodoRepo {
    async fn create(&self, input: CreateTodo) -> Result<Todo, TodoError> {
        let todo = Todo::new(Uuid::new_v4(), input.text, false);
        self.0.write().unwrap().insert(todo.id(), todo.clone());
        Ok(todo)
    }

    async fn get(&self, id: Uuid) -> Result<Todo, TodoError> {
        self.0
            .read()
            .unwrap()
            .get(&id)
            .cloned()
            .ok_or(TodoError::NotFound(id))
    }

    async fn fetch(&self, pagination: Pagination) -> Result<Vec<Todo>, TodoError> {
        let todos = self.0.read().unwrap();

        let todos = todos
            .values()
            .skip(pagination.offset.unwrap_or(0))
            .take(pagination.limit.unwrap_or(usize::MAX))
            .cloned()
            .collect::<Vec<_>>();

        Ok(todos)
    }
    async fn update(&self, id: Uuid, input: UpdateTodo) -> Result<Todo, TodoError> {
        let todo = self
            .0
            .read()
            .unwrap()
            .get(&id)
            .cloned()
            .ok_or(TodoError::NotFound(id))?;

        let text = input.text.unwrap_or_else(|| todo.text().to_owned());
        let completed = input.completed.unwrap_or(todo.completed());

        let todo = Todo::new(id, text, completed);
        self.0.write().unwrap().insert(todo.id(), todo.clone());
        Ok(todo)
    }

    async fn delete(&self, id: Uuid) -> Result<(), TodoError> {
        self.0
            .write()
            .unwrap()
            .remove(&id)
            .map(|_| ())
            .ok_or_else(|| TodoError::NotFound(id))
    }
}
