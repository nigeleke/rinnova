use jiff::civil::Date;
use thiserror::*;

use crate::domain::{MedicationId, ScriptId};

#[derive(Clone, Debug, Error)]
pub enum LogbookError {
    #[error("error.matching-medication")]
    MatchingMedication(String),

    #[error("error.duplicate-medication")]
    DuplicateMedication(MedicationId),

    #[error("error.invalid-medication")]
    InvalidMedication(MedicationId),

    #[error("error.invalid-expiry-date")]
    InvalidExpiryDate(Date),

    #[error("error.no-medications")]
    NoMedications,

    #[error("error.duplicate-script")]
    DuplicateScript(ScriptId),

    #[error("error.invalid-script")]
    InvalidScript(ScriptId),

    #[error("error.script-contains-unknown-medication")]
    UnknownMedication(MedicationId),
}
