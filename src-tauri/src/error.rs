use serde::{Deserialize, Serialize};
use std::fmt;

/// Application error type with serializable variants
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "details")]
pub enum AppError {
    Database(String),
    Validation(String),
    NotFound(String),
    Conflict(String),
    Auth(String),
    FileSystem(String),
    Import(String),
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Database(msg) => write!(f, "Database error: {}", msg),
            AppError::Validation(msg) => write!(f, "Validation error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            AppError::Auth(msg) => write!(f, "Authentication error: {}", msg),
            AppError::FileSystem(msg) => write!(f, "File system error: {}", msg),
            AppError::Import(msg) => write!(f, "Import error: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::FileSystem(err.to_string())
    }
}

/// Application result type alias
pub type AppResult<T> = Result<T, AppError>;
