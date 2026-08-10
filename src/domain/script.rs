mod id;
mod item;

pub use id::ScriptId;
pub use item::ScriptItem;

// ------------------------------------
use std::collections::HashSet;

use dioxus_i18n::tid;
use serde::{Deserialize, Serialize};

use crate::domain::{Date, LogbookError};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Script {
    id: ScriptId,
    issued_on: Date,
    expires_on: Date,
    items: HashSet<ScriptItem>,
}

impl Script {
    pub fn try_new(
        issued_on: Date,
        expires_on: Date,
        items: &[ScriptItem],
    ) -> Result<Self, LogbookError> {
        let id = ScriptId::new();
        Self::try_new_with_id(id, issued_on, expires_on, items)
    }

    pub fn try_new_with_id(
        id: ScriptId,
        issued_on: Date,
        expires_on: Date,
        items: &[ScriptItem],
    ) -> Result<Self, LogbookError> {
        Self::validate(issued_on, expires_on, items)?;

        let items = HashSet::from_iter(items.iter().copied());

        let script = Self {
            id,
            issued_on,
            expires_on,
            items,
        };

        Ok(script)
    }

    fn validate(
        issued_on: Date,
        expires_on: Date,
        items: &[ScriptItem],
    ) -> Result<(), LogbookError> {
        let mut seen = HashSet::new();

        let medication_ids = items.iter().map(|i| i.medication_id()).collect::<Vec<_>>();

        if expires_on <= issued_on {
            Err(LogbookError::InvalidExpiryDate(expires_on))
        } else if medication_ids.is_empty() {
            Err(LogbookError::NoMedications)
        } else if let Some(id) = medication_ids.iter().find(|id| !seen.insert(*id)) {
            Err(LogbookError::DuplicateMedication(*id))
        } else {
            Ok(())
        }
    }

    pub fn id(&self) -> ScriptId {
        self.id
    }

    pub fn issued_on(&self) -> Date {
        self.issued_on
    }

    pub fn expires_on(&self) -> Date {
        self.expires_on
    }

    pub fn is_valid_on(&self, date: Date) -> bool {
        self.issued_on <= date && date <= self.expires_on
    }

    pub fn items(&self) -> impl Iterator<Item = ScriptItem> + '_ {
        self.items.iter().copied()
    }
}

impl std::fmt::Display for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = tid!("script-description", issued_on: self.issued_on.to_string(), expires_on: self.expires_on.to_string(), item_count: self.items.len());
        description.fmt(f)
    }
}
