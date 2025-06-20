use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

pub type Result<T = ()> = anyhow::Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("internal error: {0}")]
    InternalError(#[from] anyhow::Error),

    #[error("internal error: {0}")]
    RedisError(#[from] fred::error::Error),

    #[error("resource not found")]
    NotFound,

    // Validation Errors
    #[error("invalid did provided")]
    InvalidDid,

    #[error("invalid handle provided")]
    InvalidHandle,
}

impl From<&AppError> for String {
    fn from(value: &AppError) -> Self {
        match value {
            AppError::NotFound => "NotFound",
            AppError::InvalidDid => "InvalidDid",
            AppError::InvalidHandle => "InvalidHandle",
            _ => "InternalError",
        }.to_string()
    }
}

impl From<&AppError> for StatusCode {
    fn from(value: &AppError) -> Self {
        match value {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::InvalidDid => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::InvalidHandle => StatusCode::UNPROCESSABLE_ENTITY,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let message = self.to_string();
        let code = (&self).into();
        let status: StatusCode = (&self).into();

        (
            status,
            Json(AppErrorResponse {
                error: code,
                message,
            }),
        ).into_response()
    }
}
