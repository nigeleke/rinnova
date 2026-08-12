mod count;
mod id;
mod item;

pub use count::SupplyCount;
pub use id::SupplyId;
pub use item::SupplyItem;

// ------------------------------------
use dioxus_i18n::tid;
use serde::{Deserialize, Serialize};

use crate::domain::{Date, MedicationId, ScriptId};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Supply {
    id: SupplyId,
    issued_on: Date,
    items: Vec<SupplyItem>,
}

impl Supply {
    pub fn new(issued_on: Date, items: &[SupplyItem]) -> Self {
        let id = SupplyId::new();
        let items = Vec::from(items);

        Self {
            id,
            issued_on,
            items,
        }
    }

    pub fn id(&self) -> SupplyId {
        self.id
    }

    pub fn issued_on(&self) -> Date {
        self.issued_on
    }

    pub fn items(&self) -> impl Iterator<Item = &SupplyItem> {
        self.items.iter()
    }

    pub fn item(&self, script_id: ScriptId, medication_id: MedicationId) -> Option<&SupplyItem> {
        self.items
            .iter()
            .find(|i| i.script_id() == script_id && i.medication_id() == medication_id)
    }
}

impl std::fmt::Display for Supply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = tid!("supply-description", issued_on: self.issued_on.to_string());
        description.fmt(f)
    }
}
