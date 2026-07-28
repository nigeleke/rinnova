use jiff::civil::Date;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{MedicationId, ScriptId};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupplyId(Uuid);

impl SupplyId {
    fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

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
