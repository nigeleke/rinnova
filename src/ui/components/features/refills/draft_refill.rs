use std::collections::HashMap;

use crate::domain::{Date, ScriptId, ScriptSnapshot, ScriptStatus, Supply, SupplyCount};

use super::DraftRefillItem;

#[derive(Clone)]
pub struct DraftRefill {
    pub issued_on: Date,
    pub items: HashMap<ScriptId, Vec<DraftRefillItem>>,
}

impl DraftRefill {
    pub fn with_scripts<'a>(&mut self, scripts: impl IntoIterator<Item = &'a ScriptSnapshot>) {
        let items = scripts
            .into_iter()
            .filter(|s| matches!(s.status(), ScriptStatus::Ok | ScriptStatus::DueToExpire))
            .fold(HashMap::new(), |mut acc, s| {
                let script_id = s.id();
                let entry = acc.entry(script_id).or_insert(Vec::new());
                s.items()
                    .iter()
                    .filter(|i| i.remaining_supplies() != SupplyCount::ZERO)
                    .for_each(|i| entry.push(DraftRefillItem::from_script_item(script_id, i)));
                acc
            });

        self.items = items;
    }

    pub fn selected_items(&self) -> impl Iterator<Item = &DraftRefillItem> {
        self.items
            .values()
            .flat_map(move |items| items.iter().filter(|i| i.selected))
    }

    pub fn update_item(&mut self, item: DraftRefillItem) {
        if let Some(entry) = self.items.get_mut(&item.script_id)
            && let Some(existing) = entry
                .iter_mut()
                .find(|i| i.medication_id == item.medication_id)
        {
            *existing = item;
        }
    }

    pub fn into_supplies(&self) -> impl Iterator<Item = Supply> {
        let issued_on = self.issued_on;

        self.selected_items().map(move |item| {
            let script_id = item.script_id;
            let medication_id = item.medication_id;
            Supply::new(script_id, medication_id, issued_on)
        })
    }
}

impl Default for DraftRefill {
    fn default() -> Self {
        Self {
            issued_on: Date::today(),
            items: Default::default(),
        }
    }
}
