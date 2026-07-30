use crate::domain::{Health, MedicationId, ScriptId};

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

impl Status {
    pub fn references_script(&self, script_id: ScriptId) -> bool {
        match self {
            Status::ScriptOk(id)
            | Status::ScriptDueToExpire(id)
            | Status::ScriptExpired(id)
            | Status::ScriptExhausted(id)
            | Status::SupplyOk(id, _)
            | Status::LastRepeat(id, _)
            | Status::NoRepeats(id, _) => id == &script_id,
            _ => false,
        }
    }

    pub fn script_health(&self) -> Option<Health> {
        match self {
            Status::ScriptOk(_) => Some(Health::Ok),
            Status::ScriptDueToExpire(_) => Some(Health::Attention),
            Status::ScriptExpired(_) => Some(Health::Critical),
            Status::ScriptExhausted(_) => Some(Health::Critical),
            Status::SupplyOk(_, _) => Some(Health::Ok),
            Status::LastRepeat(_, _) => Some(Health::Attention),
            Status::NoRepeats(_, _) => Some(Health::Critical),
            Status::MedicationOk(_) | Status::MedicationNotCovered(_) => None,
        }
    }

    pub fn references_supply(&self, script_id: ScriptId, medication_id: MedicationId) -> bool {
        match self {
            Status::SupplyOk(sid, mid)
            | Status::LastRepeat(sid, mid)
            | Status::NoRepeats(sid, mid) => sid == &script_id && mid == &medication_id,
            _ => false,
        }
    }

    pub fn supply_health(&self) -> Option<Health> {
        match self {
            Status::ScriptOk(_) | Status::ScriptDueToExpire(_) => None,
            Status::ScriptExpired(_) | Status::ScriptExhausted(_) => Some(Health::Critical),
            Status::SupplyOk(_, _) => Some(Health::Ok),
            Status::LastRepeat(_, _) => Some(Health::Attention),
            Status::NoRepeats(_, _) => Some(Health::Critical),
            Status::MedicationOk(_) | Status::MedicationNotCovered(_) => None,
        }
    }

    pub fn references_medication(&self, medication_id: MedicationId) -> bool {
        match self {
            Status::SupplyOk(_, mid)
            | Status::LastRepeat(_, mid)
            | Status::NoRepeats(_, mid)
            | Status::MedicationOk(mid)
            | Status::MedicationNotCovered(mid) => mid == &medication_id,
            _ => false,
        }
    }

    pub fn medication_health(&self) -> Option<Health> {
        match self {
            Status::ScriptOk(_)
            | Status::ScriptDueToExpire(_)
            | Status::ScriptExpired(_)
            | Status::ScriptExhausted(_) => None,
            Status::SupplyOk(_, _) => Some(Health::Ok),
            Status::LastRepeat(_, _) => Some(Health::Attention),
            Status::NoRepeats(_, _) => Some(Health::Critical),
            Status::MedicationOk(_) => Some(Health::Ok),
            Status::MedicationNotCovered(_) => Some(Health::Critical),
        }
    }
}
