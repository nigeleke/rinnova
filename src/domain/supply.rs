mod count;
mod id;
mod item;

pub use count::SupplyCount;
pub use id::SupplyId;
pub use item::SupplyItem;

// ------------------------------------
use dioxus_i18n::tid;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::domain::{Date, LogbookError, MedicationId, ScriptId};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Supply {
    id: SupplyId,
    issued_on: Date,
    items: Vec<SupplyItem>,
}

impl Supply {
    pub fn try_new(issued_on: Date, items: &[SupplyItem]) -> Result<Self, LogbookError> {
        let id = SupplyId::new();
        let items = Vec::from(items);

        let supply = Self {
            id,
            issued_on,
            items,
        };

        validate(supply)
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

fn validate(supply: Supply) -> Result<Supply, LogbookError> {
    let items = supply.items.iter().collect::<HashSet<_>>();

    if supply.items.is_empty() {
        Err(LogbookError::SupplyHasNoMedications)
    } else if items.len() != supply.items.len() {
        Err(LogbookError::SupplyHasDuplicateMedications)
    } else {
        Ok(supply)
    }
}
