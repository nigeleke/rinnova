use crate::domain::Medication;

#[derive(Clone, Default)]
pub struct DraftMedication {
    pub name: String,
    pub strength: String,
    pub notes: String,
}

impl DraftMedication {
    pub fn is_valid(&self) -> bool {
        !self.name.trim().is_empty()
    }

    pub fn into_medication(self) -> Medication {
        Medication::new(self.name, self.strength, self.notes)
    }
}
