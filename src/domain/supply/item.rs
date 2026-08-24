use serde::{Deserialize, Serialize};

use crate::domain::{MedicationId, ScriptId};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct SupplyItem {
    script_id: ScriptId,
    medication_id: MedicationId,
}

impl SupplyItem {
    pub fn new(script_id: ScriptId, medication_id: MedicationId) -> Self {
        Self {
            script_id,
            medication_id,
        }
    }

    pub fn script_id(&self) -> ScriptId {
        self.script_id
    }

    pub fn medication_id(&self) -> MedicationId {
        self.medication_id
    }
}
