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
    let mut logbook = use_signal(Logbook::default);
    let mut persistence = use_signal(PersistenceState::default);
    let mut generation = use_signal(|| 0u64);

    // Load
    use_resource(move || async move {
        match load_logbook().await {
            Ok(loaded) => {
                logbook.set(loaded);
                persistence.set(PersistenceState::Idle);
            }
            Err(error) => persistence.set(PersistenceState::Failed(error)),
        }
    });

    // Save
    use_effect(move || {
        let current = logbook();
        let current_generation = *generation.peek();

        if matches!(
            *persistence.peek(),
            PersistenceState::Idle | PersistenceState::Saving
        ) {
            // Order important:
            // generation must be updated before persistence
            let this_generation = current_generation + 1;
            generation.set(this_generation);
            persistence.set(PersistenceState::Saving);

            spawn(async move {
                match save_logbook(&current).await {
                    Ok(_) => {
                        if *generation.peek() == this_generation {
                            persistence.set(PersistenceState::Idle);
                        }
                    }
                    Err(error) => {
                        if *generation.peek() == this_generation {
                            persistence.set(PersistenceState::Failed(error));
                        }
                    }
                }
            });
        }
    });

    (logbook, persistence)
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
