use crate::domain::{Medication, MedicationId, ScriptItem, SupplyCount};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraftScriptItem {
    pub medication_id: MedicationId,
    pub selected: bool,
    pub repeats: SupplyCount,
}

impl DraftScriptItem {
    pub fn into_script_item(self) -> ScriptItem {
        ScriptItem::new(self.medication_id, self.repeats)
    }
}

impl From<&Medication> for DraftScriptItem {
    fn from(value: &Medication) -> Self {
        Self {
            medication_id: value.id(),
            selected: false,
            repeats: SupplyCount::ZERO,
        }
    }
}

impl From<&ScriptItem> for DraftScriptItem {
    fn from(value: &ScriptItem) -> Self {
        Self {
            medication_id: value.medication_id(),
            selected: true,
            repeats: value.authorised_repeats(),
        }
    }
}
