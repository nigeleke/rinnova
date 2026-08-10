use crate::domain::Health;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ScriptStatus {
    Ok,
    DueToExpire,
    NotCurrent,
    Exhausted,
}

impl ScriptStatus {
    pub fn health(&self) -> Health {
        match self {
            Self::Ok => Health::Ok,
            Self::DueToExpire => Health::Attention,
            Self::NotCurrent => Health::Critical,
            Self::Exhausted => Health::Critical,
        }
    }
}

impl std::fmt::Display for ScriptStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            ScriptStatus::Ok => "ok",
            ScriptStatus::DueToExpire => "due-to-expire",
            ScriptStatus::NotCurrent => "not-current",
            ScriptStatus::Exhausted => "exhausted",
        };
        write!(f, "script-status.{status}")
    }
}
