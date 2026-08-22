use thiserror::*;

use crate::domain::{Date, MedicationId, ScriptId, SupplyId};

#[derive(Clone, Debug, Error)]
pub enum LogbookError {
    #[error("error.internal-error")]
    IndexedDb,

    #[error("error.internal-error")]
    Serde,

    #[error("error.invalid-date")]
    InvalidDate(#[from] jiff::Error),

    #[error("error.matching-medication")]
    MatchingMedication(String),

    #[error("error.duplicate-medication")]
    DuplicateMedication(MedicationId),

    #[error("error.invalid-medication")]
    InvalidMedication(MedicationId),

    #[error("error.invalid-draft-medication")]
    InvalidDraftMedication,

    #[error("error.medication-used-in-script")]
    MedicationUsedInScript,

    #[error("error.invalid-expiry-date")]
    InvalidExpiryDate(Date),

    #[error("error.no-medications")]
    NoMedications,

    #[error("error.duplicate-script")]
    DuplicateScript(ScriptId),

    #[error("error.invalid-script")]
    InvalidScript(ScriptId),

    #[error("error.invalid-draft-script")]
    InvalidDraftScript,

    #[error("error.script-used-in-supply")]
    ScriptUsedInSupply,

    #[error("error.script-contains-unknown-medication")]
    UnknownMedication(MedicationId),

    #[error("error.duplicate-supply")]
    DuplicateSupply(SupplyId),

    #[error("error.invalid-supply")]
    InvalidSupply(SupplyId),

    #[error("error.script-out-of-date")]
    ScriptOutOfDate(ScriptId),

    #[error("error.medication-not-on-script")]
    MedicationNotOnScript(ScriptId, MedicationId),
}

impl From<&rexie::Error> for LogbookError {
    fn from(error: &rexie::Error) -> Self {
        dioxus::prelude::error!("IndexedDB error: {error}");
        Self::IndexedDb
    }
}

impl From<rexie::Error> for LogbookError {
    fn from(error: rexie::Error) -> Self {
        Self::from(&error)
    }
}

impl From<&serde_wasm_bindgen::Error> for LogbookError {
    fn from(error: &serde_wasm_bindgen::Error) -> Self {
        dioxus::prelude::error!("Serialization error: {error}");
        Self::Serde
    }
}

impl From<serde_wasm_bindgen::Error> for LogbookError {
    fn from(error: serde_wasm_bindgen::Error) -> Self {
        Self::from(&error)
    }
}
