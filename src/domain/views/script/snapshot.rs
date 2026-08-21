use crate::domain::{Date, Health, Script, ScriptId, ScriptItemSnapshot, ScriptStatus};

#[derive(Clone, PartialEq, Eq)]
pub struct ScriptSnapshot {
    script: Script,
    status: ScriptStatus,
    items: Vec<ScriptItemSnapshot>,
}

impl ScriptSnapshot {
    pub fn new(script: Script, status: ScriptStatus, items: &[ScriptItemSnapshot]) -> Self {
        Self {
            script,
            status,
            items: items.to_vec(),
        }
    }

    #[cfg(test)]
    pub fn script(&self) -> &Script {
        &self.script
    }

    pub fn id(&self) -> ScriptId {
        self.script.id()
    }

    pub fn issued_on(&self) -> Date {
        self.script.issued_on()
    }

    pub fn expires_on(&self) -> Date {
        self.script.expires_on()
    }

    pub fn is_valid(&self, as_of: Date) -> bool {
        self.issued_on() <= as_of && as_of <= self.expires_on()
    }

    pub fn status(&self) -> ScriptStatus {
        self.status
    }

    pub fn health(&self) -> Health {
        self.status.health()
    }

    pub fn items(&self) -> &[ScriptItemSnapshot] {
        &self.items
    }
}
