use thiserror::*;

use crate::domain::{MedicationId, ScriptId, SupplyId};

#[derive(Clone, Debug, Error)]
pub enum LogbookError {
    #[error("error.invalid-date")]
    InvalidDate,

    #[error("error.invalid-date-range")]
    InvalidDateRange,

    #[error("error.matching-medication")]
    MatchingMedication(String),

    #[error("error.duplicate-medication")]
    DuplicateMedication(MedicationId),

    #[error("error.invalid-medication")]
    InvalidMedication(MedicationId),

    #[error("error.medication-used-in-script")]
    MedicationUsedInScript,

    #[error("error.no-medications")]
    NoMedications,

    #[error("error.duplicate-script")]
    DuplicateScript(ScriptId),

    #[error("error.invalid-script")]
    InvalidScript(ScriptId),

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

    #[error("error.supply-has-no-medications")]
    SupplyHasNoMedications,

    #[error("error.supply-has-duplicate-medications")]
    SupplyHasDuplicateMedications,

    #[error("error.medication-not-on-script")]
    MedicationNotOnScript(ScriptId, MedicationId),
}
