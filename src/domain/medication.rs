mod id;

pub use id::MedicationId;

// ------------------------------------
use dioxus_i18n::tid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Medication {
    id: MedicationId,
    name: String,
    strength: String,
    notes: String,
}

impl Medication {
    pub fn new(name: &str, strength: &str, notes: &str) -> Self {
        let id = MedicationId::new();
        Self::with_id(id, name, strength, notes)
    }

    pub fn with_id(id: MedicationId, name: &str, strength: &str, notes: &str) -> Self {
        let name = name.trim().to_owned();
        let strength = strength.trim().to_owned();
        let notes = notes.to_owned();
        Self {
            id,
            name,
            strength,
            notes,
        }
    }

    pub fn id(&self) -> MedicationId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    pub fn strength(&self) -> &str {
        &self.strength
    }

    pub fn strength_mut(&mut self) -> &mut String {
        &mut self.strength
    }

    pub fn notes(&self) -> &str {
        &self.notes
    }

    pub fn notes_mut(&mut self) -> &mut String {
        &mut self.notes
    }

    pub fn equivalent_to(&self, other: &Self) -> bool {
        self.name.eq_ignore_ascii_case(&other.name)
            && self.strength.eq_ignore_ascii_case(&other.strength)
    }
}

impl std::fmt::Display for Medication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = &self.name;
        let strength = self.strength.trim();
        let has_strength = !self.strength.trim().is_empty();

        let description = if has_strength {
            tid!("medication-description-name-strength", name: name, strength: strength)
        } else {
            tid!("medication-description-name", name: name)
        };

        description.fmt(f)
    }
}
