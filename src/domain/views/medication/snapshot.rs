use crate::domain::{Health, Medication, MedicationStatus, SupplyCount};

pub struct MedicationSnapshot {
    medication: Medication,
    status: MedicationStatus,
    remaining_supplies: SupplyCount,
}

impl MedicationSnapshot {
    pub fn new(
        medication: Medication,
        status: MedicationStatus,
        remaining_supplies: SupplyCount,
    ) -> Self {
        Self {
            medication,
            status,
            remaining_supplies,
        }
    }

    pub fn medication(&self) -> &Medication {
        &self.medication
    }

    pub fn status(&self) -> MedicationStatus {
        self.status
    }

    pub fn health(&self) -> Health {
        self.status.health()
    }

    pub fn remaining_supplies(&self) -> SupplyCount {
        self.remaining_supplies
    }
}
