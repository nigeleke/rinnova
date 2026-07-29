use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupplyId(Uuid);

impl SupplyId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}
