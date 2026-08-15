use crate::domain::{Medication, ScriptItem, SupplyCount};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DraftScriptItem {
    pub medication: Medication,
    pub selected: bool,
    pub repeats: SupplyCount,
}

impl DraftScriptItem {
    pub fn new(medication: &Medication, selected: bool, repeats: SupplyCount) -> Self {
        Self {
            medication: medication.clone(),
            selected,
            repeats,
        }
    }

    pub fn into_script_item(self) -> ScriptItem {
        ScriptItem::new(self.medication.id(), self.repeats)
    }
}

impl From<&Medication> for DraftScriptItem {
    fn from(value: &Medication) -> Self {
        Self::new(value, false, SupplyCount::ZERO)
    }
}
