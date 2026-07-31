use crate::domain::{Medication, MedicationId};

#[derive(Clone, Default, PartialEq, Eq)]
pub struct DraftMedication {
    id: Option<MedicationId>,
    pub name: String,
    pub strength: String,
    pub notes: String,
}

impl DraftMedication {
    pub fn is_valid(&self) -> bool {
        !self.name.trim().is_empty()
    }
}

impl From<&Medication> for DraftMedication {
    fn from(medication: &Medication) -> Self {
        Self {
            id: Some(medication.id()),
            name: medication.name().to_owned(),
            strength: medication.strength().to_owned(),
            notes: medication.notes().to_owned(),
        }
    }
}
