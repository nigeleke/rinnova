use crate::domain::Health;

#[derive(Clone, Copy)]
pub enum MedicationStatus {
    Ok,
    LastRepeat,
    NoRepeats,
}

impl MedicationStatus {
    pub fn health(&self) -> Health {
        match self {
            Self::Ok => Health::Ok,
            Self::LastRepeat => Health::Attention,
            Self::NoRepeats => Health::Critical,
        }
    }
}
