use crate::domain::Health;

#[derive(Clone, Copy)]
pub enum ScriptStatus {
    ScriptOk,
    ScriptDueToExpire,
    ScriptNotCurrent,
    ScriptExhausted,
}

impl ScriptStatus {
    pub fn health(&self) -> Health {
        match self {
            Self::ScriptOk => Health::Ok,
            Self::ScriptDueToExpire => Health::Attention,
            Self::ScriptNotCurrent => Health::Critical,
            Self::ScriptExhausted => Health::Critical,
        }
    }
}
