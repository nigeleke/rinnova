use crate::domain::{MedicationId, ScriptId, ScriptItemSnapshot, ScriptItemStatus};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DraftRefillItem {
    pub script_id: ScriptId,
    pub medication_id: MedicationId,
    pub status: ScriptItemStatus,
    pub selected: bool,
}

impl DraftRefillItem {
    pub fn from_script_item(script_id: ScriptId, item: &ScriptItemSnapshot) -> Self {
        let medication_id = item.medication_id();
        let status = item.status();
        let selected = false;

        Self {
            script_id,
            medication_id,
            status,
            selected,
        }
    }
}
