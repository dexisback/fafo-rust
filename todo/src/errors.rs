use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::Serialize;

#[derive(Debug)]
pub enum ApiError {
    TodoNotFound,
    InvalidInput(String),
    InternalServerError,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        tracing::error!("database error: {error}");
        ApiError::InternalServerError
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::TodoNotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "TODO not found".to_string(),
                }),
            )
                .into_response(),

            ApiError::InvalidInput(message) => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { error: message }),
            )
                .into_response(),

            ApiError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Internal server error".to_string(),
                }),
            )
                .into_response(),
        }
    }
}
