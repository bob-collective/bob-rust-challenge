use thiserror::Error;

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("Key not found: {0}")]
    KeyNotFound(String),

    #[error("Invalid signature")]
    InvalidSignature,

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, StoreError>;
