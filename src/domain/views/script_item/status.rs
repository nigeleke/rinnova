use crate::domain::Health;

#[derive(Clone, Copy, PartialEq, Eq)]
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

impl std::fmt::Display for ScriptItemStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            Self::SupplyOk => "ok",
            Self::LastRepeat => "last-repeat",
            Self::NoRepeats => "no-repeats",
        };
        write!(f, "script-item-status.{status}")
    }
}
