use serde::{Deserialize, Serialize};

use crate::domain::{MedicationId, SupplyCount};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct ScriptItem {
    medication_id: MedicationId,
    authorised_repeats: SupplyCount,
}

impl ScriptItem {
    pub fn new(medication_id: MedicationId, authorised_repeats: SupplyCount) -> Self {
        Self {
            medication_id,
            authorised_repeats,
        }
    }

    pub fn medication_id(&self) -> MedicationId {
        self.medication_id
    }

    pub fn authorised_repeats(&self) -> SupplyCount {
        self.authorised_repeats
    }
}
