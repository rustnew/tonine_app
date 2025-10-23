use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;
use sqlx::error::Error as SqlxError;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(String),
    NotFound(String),
    ValidationError(String),
    AuthenticationError(String),
    InternalServerError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::DatabaseError(msg) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "DATABASE_ERROR".to_string(),
                    message: msg.to_string(),
                })
            }
            AppError::NotFound(msg) => {
                HttpResponse::NotFound().json(ErrorResponse {
                    error: "NOT_FOUND".to_string(),
                    message: msg.to_string(),
                })
            }
            AppError::ValidationError(msg) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    error: "VALIDATION_ERROR".to_string(),
                    message: msg.to_string(),
                })
            }
            AppError::AuthenticationError(msg) => {
                HttpResponse::Unauthorized().json(ErrorResponse {
                    error: "AUTHENTICATION_ERROR".to_string(),
                    message: msg.to_string(),
                })
            }
            AppError::InternalServerError(msg) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "INTERNAL_ERROR".to_string(),
                    message: msg.to_string(),
                })
            }
        }
    }
}

impl From<SqlxError> for AppError {
    fn from(error: SqlxError) -> Self {
        AppError::DatabaseError(error.to_string())
    }
}