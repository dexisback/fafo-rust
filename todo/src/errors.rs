use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::Serialize;

#[derive(Debug)]
pub enum ApiError {
    TodoNotFound,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::TodoNotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "todo nahi mila".to_string(),
                }),
            )
                .into_response(),
        }
    }
}
