use crate::domain::{LogbookError, Medication, MedicationId};

#[derive(Clone, Default, PartialEq, Eq)]
pub struct DraftMedication {
    pub id: Option<MedicationId>,
    pub name: String,
    pub strength: String,
    pub notes: String,
}

impl DraftMedication {
    pub fn is_valid(&self) -> bool {
        !self.name.trim().is_empty()
    }

    pub fn try_into_medication(self) -> Result<Medication, LogbookError> {
        if self.is_valid() {
            let medication = match self.id {
                Some(id) => Medication::with_id(id, &self.name, &self.strength, &self.notes),
                None => Medication::new(&self.name, &self.strength, &self.notes),
            };
            Ok(medication)
        } else {
            Err(LogbookError::InvalidDraftMedication)
        }
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
