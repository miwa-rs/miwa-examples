use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    Todo(#[from] TodoError),
}

#[allow(dead_code)]
#[derive(thiserror::Error, Debug)]
pub enum TodoError {
    #[error(transparent)]
    Generic(anyhow::Error),
    #[error("Todo {0} not found.")]
    NotFound(Uuid),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        tracing::error!("Error on request: {}", self);

        let error_message = self.to_string();
        let status = match self {
            ApiError::Todo(TodoError::Generic(_)) => internal_server_error(),
            ApiError::Todo(TodoError::NotFound(_)) => not_found(),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

fn internal_server_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}
