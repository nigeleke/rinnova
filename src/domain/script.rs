use std::collections::HashSet;

use jiff::civil::Date;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{LogbookError, MedicationId};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptId(Uuid);

impl ScriptId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct ScriptItem {
    medication_id: MedicationId,
    authorised_repeats: usize,
}

impl ScriptItem {
    pub fn new(medication_id: MedicationId, authorised_repeats: usize) -> Self {
        Self {
            medication_id,
            authorised_repeats,
        }
    }

    pub fn medication_id(&self) -> MedicationId {
        self.medication_id
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
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
        Self::validate(issued_on, expires_on, items)?;

        let id = ScriptId::new();
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

        let medication_ids = items.iter().map(|i| i.medication_id).collect::<Vec<_>>();

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

    pub fn items(&self) -> impl Iterator<Item = ScriptItem> + '_ {
        self.items.iter().copied()
    }

    pub fn authorised_supplies(&self, medication_id: MedicationId) -> usize {
        self.items()
            .filter_map(|i| (i.medication_id == medication_id).then_some(i.authorised_repeats))
            .sum::<usize>()
            + 1usize
    }
}
