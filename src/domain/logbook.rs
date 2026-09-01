use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::domain::{
    Date, LogbookError, Medication, MedicationId, Period, Script, ScriptId, Supply, SupplyId,
    SupplyItem,
};

#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Logbook {
    medications: HashMap<MedicationId, Medication>,
    scripts: HashMap<ScriptId, Script>,
    supplies: HashMap<SupplyId, Supply>,
}

enum Validation {
    Add,
    Update,
}

impl Logbook {
    pub fn medications(&self) -> impl Iterator<Item = &Medication> {
        self.medications.values()
    }

    pub fn medication(&self, id: MedicationId) -> Option<&Medication> {
        self.medications.get(&id)
    }

    pub fn try_add_medication(&mut self, medication: Medication) -> Result<(), LogbookError> {
        let id = medication.id();
        self.validate_medication(&medication, Validation::Add)?;
        self.medications.insert(id, medication);
        Ok(())
    }

    pub fn try_update_medication(&mut self, medication: Medication) -> Result<(), LogbookError> {
        let id = medication.id();

        let existing = self
            .medications
            .get(&id)
            .ok_or(LogbookError::MedicationNotFound(id))?;

        let update_allowed = !self.is_medication_locked(id) || {
            let mut existing_with_new_notes = existing.clone();
            existing_with_new_notes.set_notes(medication.notes());
            existing_with_new_notes == medication
        };

        if update_allowed {
            self.validate_medication(&medication, Validation::Update)?;
            self.medications.insert(id, medication);
            Ok(())
        } else {
            Err(LogbookError::MedicationUsedInScript)
        }
    }

    pub fn try_remove_medication(&mut self, id: MedicationId) -> Result<(), LogbookError> {
        if self.is_medication_locked(id) {
            Err(LogbookError::MedicationUsedInScript)
        } else {
            self.medications
                .remove(&id)
                .map(|_| ())
                .ok_or(LogbookError::MedicationNotFound(id))
        }
    }

    fn validate_medication(
        &self,
        medication: &Medication,
        validation: Validation,
    ) -> Result<(), LogbookError> {
        let duplicate = matches!(validation, Validation::Add)
            && self.medications.contains_key(&medication.id());

        let matching = self.medications.values().find(|existing| {
            existing.id() != medication.id() && existing.equivalent_to(medication)
        });

        if duplicate {
            Err(LogbookError::DuplicateMedication(medication.id()))
        } else if let Some(existing) = matching {
            Err(LogbookError::MatchingMedication(existing.name().to_owned()))
        } else {
            Ok(())
        }
    }

    pub fn is_medication_locked(&self, id: MedicationId) -> bool {
        self.scripts
            .values()
            .any(|s| s.items().any(|i| i.medication_id() == id))
    }

    pub fn scripts(&self) -> impl Iterator<Item = &Script> {
        self.scripts.values()
    }

    pub fn script(&self, id: ScriptId) -> Option<&Script> {
        self.scripts.get(&id)
    }

    pub fn try_add_script(&mut self, script: Script) -> Result<(), LogbookError> {
        let id = script.id();
        self.validate_script(&script, Validation::Add)?;
        self.scripts.insert(id, script);
        Ok(())
    }

    pub fn try_update_script(&mut self, script: Script) -> Result<(), LogbookError> {
        let id = script.id();

        if self.is_script_immutable(id) {
            Err(LogbookError::ScriptUsedInSupply)
        } else {
            self.validate_script(&script, Validation::Update)?;
            self.scripts
                .get_mut(&id)
                .map(|existing| *existing = script)
                .ok_or(LogbookError::ScriptNotFound(id))
        }
    }

    pub fn try_remove_script(&mut self, id: ScriptId) -> Result<(), LogbookError> {
        if self.is_script_immutable(id) {
            Err(LogbookError::ScriptUsedInSupply)
        } else {
            self.scripts
                .remove(&id)
                .map(|_| ())
                .ok_or(LogbookError::ScriptNotFound(id))
        }
    }

    fn validate_script(&self, script: &Script, validation: Validation) -> Result<(), LogbookError> {
        let duplicate =
            matches!(validation, Validation::Add) && self.scripts.contains_key(&script.id());

        let unknown_medication = script
            .items()
            .find(|i| self.medication(i.medication_id()).is_none());

        if duplicate {
            Err(LogbookError::DuplicateScript(script.id()))
        } else if let Some(item) = unknown_medication {
            Err(LogbookError::UnknownMedication(item.medication_id()))
        } else {
            Ok(())
        }
    }

    pub fn is_script_immutable(&self, id: ScriptId) -> bool {
        self.supplies
            .values()
            .any(|s| s.items().any(|i| i.script_id() == id))
    }

    pub fn supplies(&self) -> impl Iterator<Item = &Supply> {
        self.supplies.values()
    }

    pub fn supply(&self, supply_id: SupplyId) -> Option<&Supply> {
        self.supplies.get(&supply_id)
    }

    pub fn try_add_supply(&mut self, supply: Supply) -> Result<(), LogbookError> {
        let id = supply.id();
        self.validate_supply(&supply, Validation::Add)?;
        self.supplies.insert(id, supply);
        Ok(())
    }

    // Note: no try_update_supply...

    pub fn try_remove_supply(&mut self, id: SupplyId) -> Result<(), LogbookError> {
        match self.supplies.remove(&id) {
            Some(_) => Ok(()),
            None => Err(LogbookError::SupplyNotFound(id)),
        }
    }

    fn validate_supply(&self, supply: &Supply, validation: Validation) -> Result<(), LogbookError> {
        let supply_id = supply.id();
        let issued_on = supply.issued_on();

        let duplicate =
            matches!(validation, Validation::Add) && self.supplies.contains_key(&supply_id);

        if duplicate {
            Err(LogbookError::DuplicateSupply(supply_id))
        } else {
            supply
                .items()
                .try_for_each(|item| self.validate_supply_item(item, issued_on))
        }
    }

    fn validate_supply_item(&self, item: &SupplyItem, issued_on: Date) -> Result<(), LogbookError> {
        let script_id = item.script_id();
        let medication_id = item.medication_id();

        let script = self
            .script(script_id)
            .ok_or(LogbookError::ScriptNotFound(script_id))?;

        script
            .is_valid_on(issued_on)
            .then_some(())
            .ok_or(LogbookError::ScriptOutOfDate(script_id))?;

        let medication = self
            .medication(medication_id)
            .ok_or(LogbookError::MedicationNotFound(medication_id))?;

        script
            .items()
            .any(|i| i.medication_id() == medication.id())
            .then_some(())
            .ok_or(LogbookError::MedicationNotOnScript(
                script_id,
                medication_id,
            ))
    }

    pub fn housekeeping(&mut self, as_of: Date) -> Result<bool, LogbookError> {
        let old_script_ids = self
            .scripts()
            .filter_map(|s| {
                (s.expires_on() + Period::retention_period() <= as_of).then_some(s.id())
            })
            .collect::<HashSet<_>>();

        let old_supply_ids = self
            .supplies()
            .filter_map(|s| {
                s.items()
                    .all(|i| old_script_ids.contains(&i.script_id()))
                    .then_some(s.id())
            })
            .collect::<HashSet<_>>();

        old_supply_ids
            .iter()
            .try_for_each(|id| self.try_remove_supply(*id))
            .and_then(|_| {
                old_script_ids
                    .iter()
                    .try_for_each(|id| self.try_remove_script(*id))
            })?;

        let items_deleted = old_supply_ids.len() + old_script_ids.len() > 0;

        Ok(items_deleted)
    }
}

#[coverage(off)]
#[cfg(test)]
mod tests;
