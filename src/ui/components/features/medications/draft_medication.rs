use crate::domain::{LogbookError, Medication, MedicationId};

#[derive(Clone, Default, PartialEq, Eq)]
pub struct DraftMedication {
    pub id: Option<MedicationId>,
    pub name: String,
    pub strength: String,
    pub notes: String,
}

impl DraftMedication {
    pub fn try_into_medication(self) -> Result<Medication, LogbookError> {
        match self.id {
            Some(id) => Medication::try_new_with_id(id, &self.name, &self.strength, &self.notes),
            None => Medication::try_new(&self.name, &self.strength, &self.notes),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.clone().try_into_medication().is_ok()
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
