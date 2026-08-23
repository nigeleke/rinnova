use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum StorageError {
    #[error("error.internal-error")]
    IndexedDb,

    #[error("error.internal-error")]
    Serde,
}

impl From<&rexie::Error> for StorageError {
    fn from(error: &rexie::Error) -> Self {
        dioxus::prelude::error!("IndexedDB error: {error}");
        Self::IndexedDb
    }
}

impl From<rexie::Error> for StorageError {
    fn from(error: rexie::Error) -> Self {
        Self::from(&error)
    }
}

impl From<&serde_wasm_bindgen::Error> for StorageError {
    fn from(error: &serde_wasm_bindgen::Error) -> Self {
        dioxus::prelude::error!("Serialization error: {error}");
        Self::Serde
    }
}

impl From<serde_wasm_bindgen::Error> for StorageError {
    fn from(error: serde_wasm_bindgen::Error) -> Self {
        Self::from(&error)
    }
}
