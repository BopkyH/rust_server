use actix_web::{HttpResponse, ResponseError};
use diesel::result;
use r2d2;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    DbError(#[from] result::Error),

    #[error("Connection pool error: {0}")]
    PoolError(#[from] r2d2::Error),

    #[error("Not found")]
    NotFound,

    #[error("Validation errors: {0}")]
    ValidationJsonError(serde_json::Value),

    #[error("Internal server error")]
    InternalError,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::DbError(e) => {
                HttpResponse::InternalServerError().body(format!("Database error: {}", e))
            }
            ApiError::PoolError(e) => {
                HttpResponse::InternalServerError().body(format!("Pool error: {}", e))
            }
            ApiError::NotFound => HttpResponse::NotFound().body("Resource not found"),
            ApiError::ValidationJsonError(json) => {
                HttpResponse::BadRequest().json(json)
            }
            ApiError::InternalError => {
                HttpResponse::InternalServerError().body("Internal server error")
            }
        }
    }
}
