use crate::domain::{Date, LogbookError, Medication, Period, Script, ScriptId};

use super::DraftScriptItem;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DraftScript {
    pub id: Option<ScriptId>,
    pub issued_on: Date,
    pub expires_on: Date,
    pub items: Vec<DraftScriptItem>,
}

impl DraftScript {
    pub fn new<'a>(medications: impl Iterator<Item = &'a Medication>) -> Self {
        let mut medications = medications.cloned().collect::<Vec<_>>();
        medications.sort_by(|a, b| a.name().cmp(b.name()));

        let id = None;
        let issued_on = Date::today();
        let expires_on = issued_on + Period::one_year();
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

        script.items().for_each(|script_item| {
            if let Some(draft_item) = self
                .items
                .iter_mut()
                .find(|i| i.medication.id() == script_item.medication_id())
            {
                draft_item.selected = true;
                draft_item.repeats = script_item.authorised_repeats();
            };
        });

        self
    }

    pub fn try_into_script(self) -> Result<Script, LogbookError> {
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
    }

    pub fn is_valid(&self) -> bool {
        self.clone().try_into_script().is_ok()
    }

    pub fn update_item(&mut self, item: DraftScriptItem) {
        if let Some(existing) = self
            .items
            .iter_mut()
            .find(|i| i.medication.id() == item.medication.id())
        {
            *existing = item;
        }
    }
}
