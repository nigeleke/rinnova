use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct MedicationId(Uuid);

impl MedicationId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl std::fmt::Display for MedicationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Medication {
    id: MedicationId,
    name: String,
    strength: String,
    notes: String,
}

impl Medication {
    pub fn new(name: &str, strength: &str, notes: &str) -> Self {
        let id = MedicationId::new();
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

    pub fn equivalent_to(&self, other: &Self) -> bool {
        self.name.eq_ignore_ascii_case(&other.name)
            && self.strength.eq_ignore_ascii_case(&other.strength)
    }
}

impl std::fmt::Display for Medication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.strength.is_empty() {
            write!(f, "{}", self.name)
        } else {
            write!(f, "{} ({})", self.name, self.strength)
        }
    }
}
