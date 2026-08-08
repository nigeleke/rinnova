use crate::domain::{Health, Script, ScriptItemSnapshot, ScriptStatus};

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

    pub fn script(&self) -> &Script {
        &self.script
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
