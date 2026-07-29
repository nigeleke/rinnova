use crate::domain::{MedicationId, ScriptId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    ScriptOk(ScriptId),
    ScriptDueToExpire(ScriptId),
    ScriptExpired(ScriptId),
    ScriptExhausted(ScriptId),
    SupplyOk(ScriptId, MedicationId),
    LastRepeat(ScriptId, MedicationId),
    NoRepeats(ScriptId, MedicationId),
    MedicationOk(MedicationId),
    MedicationNotCovered(MedicationId),
}
