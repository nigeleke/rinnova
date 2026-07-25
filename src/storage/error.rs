use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("IndexedDB error: {0}")]
    IndexedDb(#[from] rexie::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type StorageResult<T> = Result<T, StorageError>;
