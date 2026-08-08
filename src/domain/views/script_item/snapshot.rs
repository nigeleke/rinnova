use crate::domain::{Health, Medication, ScriptItemStatus, SupplyCount};

#[derive(Clone)]
pub struct ScriptItemSnapshot {
    medication: Medication,
    remaining_supplies: SupplyCount,
    status: ScriptItemStatus,
}

impl ScriptItemSnapshot {
    pub fn new(
        medication: Medication,
        remaining_supplies: SupplyCount,
        status: ScriptItemStatus,
    ) -> Self {
        Self {
            medication,
            remaining_supplies,
            status,
        }
    }

    pub fn medication(&self) -> &Medication {
        &self.medication
    }

    pub fn remaining_supplies(&self) -> SupplyCount {
        self.remaining_supplies
    }

    pub fn status(&self) -> ScriptItemStatus {
        self.status
    }

    pub fn health(&self) -> Health {
        self.status.health()
    }
}
