use dioxus::prelude::*;
use rexie::{ObjectStore, Rexie};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::domain::Logbook;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("IndexedDB error: {0}")]
    IndexedDb(#[from] rexie::Error),
}

pub type StorageResult<T> = Result<T, StorageError>;

pub fn use_logbook() -> Signal<Logbook> {
    let logbook = use_signal(Logbook::default);

    use_resource({
        let mut logbook = logbook.clone();

        move || async move {
            if let Ok(loaded) = load_logbook().await {
                logbook.set(loaded);
            }
        }
    });

    logbook
}

pub async fn load_logbook() -> StorageResult<Logbook> {
    Ok(Logbook::default())
}
