use dioxus::prelude::*;
use rexie::{ObjectStore, Rexie, TransactionMode};

use crate::domain::Logbook;
use crate::storage::StorageError;

#[derive(Clone, Default)]
pub enum PersistenceState {
    #[default]
    Loading,
    Idle,
    Saving,
    Failed(StorageError),
}

pub fn use_logbook() -> (Signal<Logbook>, Signal<PersistenceState>) {
    let logbook = use_signal(Logbook::default);
    let mut persistence = use_signal(PersistenceState::default);

    // Load
    use_resource(move || load_logbook_and_apply(logbook, persistence));

    // Save
    use_effect(move || {
        let _ = logbook();
        if matches!(*persistence.peek(), PersistenceState::Idle) {
            persistence.set(PersistenceState::Saving);
            spawn(save_logbook(logbook, persistence));
        }
    });

    (logbook, persistence)
}

async fn load_logbook_and_apply(
    mut logbook: Signal<Logbook>,
    mut persistence: Signal<PersistenceState>,
) {
    match load_logbook_database().await {
        Ok(loaded) => {
            logbook.set(loaded);
            persistence.set(PersistenceState::Idle);
        }
        Err(error) => {
            persistence.set(PersistenceState::Failed(error));
        }
    }
}

async fn save_logbook(logbook: Signal<Logbook>, mut persistence: Signal<PersistenceState>) {
    loop {
        let snapshot = logbook.read().clone();

        match save_logbook_database(&snapshot).await {
            Ok(_) if snapshot == logbook.read().clone() => {
                persistence.set(PersistenceState::Idle);
                break;
            }
            Ok(_) => {}
            Err(error) => {
                persistence.set(PersistenceState::Failed(error));
                break;
            }
        }
    }
}

const DATABASE_NAME: &str = "rinnova";
const DATABASE_VERSION: u32 = 1;
const LOGBOOK_STORE: &str = "logbook";
const LOGBOOK_KEY: &str = "current";

async fn open_database() -> Result<Rexie, StorageError> {
    Rexie::builder(DATABASE_NAME)
        .version(DATABASE_VERSION)
        .add_object_store(ObjectStore::new(LOGBOOK_STORE))
        .build()
        .await
        .map_err(|e| StorageError::from(&e))
}

async fn load_logbook_database() -> Result<Logbook, StorageError> {
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

async fn save_logbook_database(logbook: &Logbook) -> Result<(), StorageError> {
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
