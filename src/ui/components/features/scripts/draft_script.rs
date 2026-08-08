use crate::domain::{Date, LogbookError, Medication, Script, ScriptId};

use super::DraftScriptItem;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DraftScript {
    pub id: Option<ScriptId>,
    pub issued_on: Date,
    pub expires_on: Date,
    pub items: Vec<DraftScriptItem>,
}

impl DraftScript {
    pub fn new(medications: &[Medication]) -> Self {
        let mut medications = medications.to_vec();
        medications.sort_by(|a, b| a.to_string().cmp(&b.to_string()));

        let id = None;
        let issued_on = Date::today();
        let expires_on = issued_on.plus_years(1);
        let items = medications
            .iter()
            .map(DraftScriptItem::from)
            .collect::<Vec<_>>();
        Self {
            id,
            issued_on,
            expires_on,
            items,
        }
    }

    pub fn using_script(mut self, script: &Script) -> Self {
        self.id = Some(script.id());
        self.issued_on = script.issued_on();
        self.expires_on = script.expires_on();
        script
            .items()
            .map(|item| DraftScriptItem::from(&item))
            .for_each(|item| self.update_item(item));

        self
    }

    pub fn is_valid(&self) -> bool {
        let issued_before_expires = self.issued_on < self.expires_on;
        let selected_items = self.items.iter().filter(|i| i.selected);
        issued_before_expires && selected_items.count() > 0
    }

    pub fn try_into_script(self) -> Result<Script, LogbookError> {
        if self.is_valid() {
            let issued_on = self.issued_on;
            let expires_on = self.expires_on;
            let items = self
                .items
                .into_iter()
                .filter_map(|i| i.selected.then_some(i.into_script_item()))
                .collect::<Vec<_>>();
            match self.id {
                Some(id) => Script::try_new_with_id(id, issued_on, expires_on, &items),
                None => Script::try_new(issued_on, expires_on, &items),
            }
        } else {
            Err(LogbookError::InvalidDraftScript)
        }
    }

    pub fn update_item(&mut self, item: DraftScriptItem) {
        if let Some(existing) = self
            .items
            .iter_mut()
            .find(|i| i.medication_id == item.medication_id)
        {
            *existing = item;
        }
    }
}
