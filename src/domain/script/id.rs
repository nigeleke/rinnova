use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptId(Uuid);

impl ScriptId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}
