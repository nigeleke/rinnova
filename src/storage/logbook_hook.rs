use dioxus::prelude::*;
use rexie::{ObjectStore, Rexie, TransactionMode};
use thiserror::Error;

use crate::domain::Logbook;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error(transparent)]
    IndexedDb(#[from] rexie::Error),

    #[error(transparent)]
    Serde(#[from] serde_wasm_bindgen::Error),
}

pub type StorageResult<T> = Result<T, StorageError>;

pub fn use_logbook() -> Signal<Logbook> {
    let mut logbook = use_signal(Logbook::default);

    use_resource(move || async move {
        match load_logbook().await {
            Ok(loaded) => logbook.set(loaded),
            Err(error) => error!("Failed to load logbook: {error}"),
        }
    });

    use_effect(move || {
        let current = logbook.read().clone();

        spawn(async move {
            if let Err(error) = save_logbook(&current).await {
                error!("Failed to save logbook: {error}");
            }
        });
    });

    logbook
}

const DATABASE_NAME: &str = "scriptpilot";
const DATABASE_VERSION: u32 = 1;
const LOGBOOK_STORE: &str = "logbook";
const LOGBOOK_KEY: &str = "current";

async fn open_database() -> Result<Rexie, StorageError> {
    Ok(Rexie::builder(DATABASE_NAME)
        .version(DATABASE_VERSION)
        .add_object_store(ObjectStore::new(LOGBOOK_STORE))
        .build()
        .await?)
}

pub async fn load_logbook() -> Result<Logbook, StorageError> {
    let db = open_database().await?;

    let tx = db.transaction(&[LOGBOOK_STORE], TransactionMode::ReadOnly)?;
    let store = tx.store(LOGBOOK_STORE)?;

    let value = store.get(LOGBOOK_KEY.into()).await?;

    tx.done().await?;

    match value {
        Some(value) => Ok(serde_wasm_bindgen::from_value(value)?),
        None => Ok(Logbook::default()),
    }
}

pub async fn save_logbook(logbook: &Logbook) -> Result<(), StorageError> {
    let db = open_database().await?;

    let tx = db.transaction(&[LOGBOOK_STORE], TransactionMode::ReadWrite)?;
    let store = tx.store(LOGBOOK_STORE)?;

    store
        .put(
            &serde_wasm_bindgen::to_value(logbook)?,
            Some(&LOGBOOK_KEY.into()),
        )
        .await?;

    tx.done().await?;

    Ok(())
}
