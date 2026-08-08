use crate::domain::Health;

#[derive(Clone, Copy, PartialEq, Eq)]
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

impl std::fmt::Display for MedicationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            Self::Ok => "ok",
            Self::LastRepeat => "last-repeat",
            Self::NoRepeats => "no-repeats",
        };
        write!(f, "medication-status.{status}")
    }
}
