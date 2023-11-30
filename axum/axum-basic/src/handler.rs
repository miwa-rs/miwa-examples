use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    error::ApiError,
    model::{CreateTodo, Pagination, Todo, UpdateTodo},
    repo::TodoRepoImpl,
};

pub async fn todos_update(
    Path(id): Path<Uuid>,
    State(repo): State<TodoRepoImpl>,
    Json(input): Json<UpdateTodo>,
) -> Result<Json<Todo>, ApiError> {
    repo.update(id, input)
        .await
        .map(Json)
        .map_err(ApiError::from)
}

pub async fn todos_get(
    Path(id): Path<Uuid>,
    State(repo): State<TodoRepoImpl>,
) -> Result<Json<Todo>, ApiError> {
    repo.get(id).await.map(Json).map_err(ApiError::from)
}

pub async fn todos_delete(
    Path(id): Path<Uuid>,
    State(repo): State<TodoRepoImpl>,
) -> impl IntoResponse {
    repo.delete(id).await.map_err(ApiError::from)
}

pub async fn todos_create(
    State(repo): State<TodoRepoImpl>,
    Json(input): Json<CreateTodo>,
) -> impl IntoResponse {
    repo.create(input).await.map(Json).map_err(ApiError::from)
}

pub async fn todos_index(
    pagination: Option<Query<Pagination>>,
    State(repo): State<TodoRepoImpl>,
) -> impl IntoResponse {
    let Query(pagination) = pagination.unwrap_or_default();
    repo.fetch(pagination)
        .await
        .map(Json)
        .map_err(ApiError::from)
}
