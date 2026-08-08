mod count;
mod id;

pub use count::SupplyCount;
pub use id::SupplyId;

// ------------------------------------
use serde::{Deserialize, Serialize};

use crate::domain::{Date, MedicationId, ScriptId};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Supply {
    id: SupplyId,
    script_id: ScriptId,
    medication_id: MedicationId,
    issued_on: Date,
}

impl Supply {
    pub fn new(script_id: ScriptId, medication_id: MedicationId, issued_on: Date) -> Self {
        let id = SupplyId::new();
        Self {
            id,
            script_id,
            medication_id,
            issued_on,
        }
    }

    pub fn id(&self) -> SupplyId {
        self.id
    }

    pub fn script_id(&self) -> ScriptId {
        self.script_id
    }

    pub fn medication_id(&self) -> MedicationId {
        self.medication_id
    }

    pub fn issued_on(&self) -> Date {
        self.issued_on
    }
}
