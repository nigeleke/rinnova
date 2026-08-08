use crate::domain::Health;

#[derive(Clone, Copy)]
pub enum ScriptItemStatus {
    SupplyOk,
    LastRepeat,
    NoRepeats,
}

impl ScriptItemStatus {
    pub fn health(&self) -> Health {
        match self {
            Self::SupplyOk => Health::Ok,
            Self::LastRepeat => Health::Attention,
            Self::NoRepeats => Health::Critical,
        }
    }
}
