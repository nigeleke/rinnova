use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::domain::{
    Date, LogbookError, Medication, MedicationId, Period, Script, ScriptId, Supply, SupplyId,
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

        if self.is_medication_immutable(id) {
            Err(LogbookError::MedicationUsedInScript)
        } else {
            self.validate_medication(&medication, Validation::Update)?;
            self.medications
                .get_mut(&id)
                .map(|existing| *existing = medication)
                .ok_or(LogbookError::InvalidMedication(id))
        }
    }

    pub fn try_remove_medication(&mut self, id: MedicationId) -> Result<(), LogbookError> {
        if self.is_medication_immutable(id) {
            Err(LogbookError::MedicationUsedInScript)
        } else {
            self.medications
                .remove(&id)
                .map(|_| ())
                .ok_or(LogbookError::InvalidMedication(id))
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

    pub fn is_medication_immutable(&self, id: MedicationId) -> bool {
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
                .ok_or(LogbookError::InvalidScript(id))
        }
    }

    pub fn try_remove_script(&mut self, id: ScriptId) -> Result<(), LogbookError> {
        if self.is_script_immutable(id) {
            Err(LogbookError::ScriptUsedInSupply)
        } else {
            self.scripts
                .remove(&id)
                .map(|_| ())
                .ok_or(LogbookError::InvalidScript(id))
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
            None => Err(LogbookError::InvalidSupply(id)),
        }
    }

    fn validate_supply(&self, supply: &Supply, validation: Validation) -> Result<(), LogbookError> {
        let supply_id = supply.id();
        let issued_on = supply.issued_on();

        let duplicate =
            matches!(validation, Validation::Add) && self.supplies.contains_key(&supply_id);

        let unknown_script = supply
            .items()
            .find(|i| self.script(i.script_id()).is_none());

        let out_of_date_script = supply.items().find(|i| {
            self.script(i.script_id())
                .is_some_and(|s| !s.is_valid_on(issued_on))
        });

        let unknown_medication = supply
            .items()
            .find(|i| self.medication(i.medication_id()).is_none());

        let invalid_medication = supply.items().find(|i| {
            match (
                self.script(i.script_id()),
                self.medication(i.medication_id()),
            ) {
                (Some(script), Some(medication)) => {
                    !script.items().any(|i| i.medication_id() == medication.id())
                }
                (_, _) => false,
            }
        });

        if duplicate {
            Err(LogbookError::DuplicateSupply(supply.id()))
        } else if let Some(item) = unknown_script {
            Err(LogbookError::InvalidScript(item.script_id()))
        } else if let Some(item) = out_of_date_script {
            Err(LogbookError::ScriptOutOfDate(item.script_id()))
        } else if let Some(item) = unknown_medication {
            Err(LogbookError::InvalidMedication(item.medication_id()))
        } else if let Some(item) = invalid_medication {
            Err(LogbookError::MedicationNotOnScript(
                item.script_id(),
                item.medication_id(),
            ))
        } else {
            Ok(())
        }
    }

    pub fn housekeeping(&mut self, as_of: Date) -> Result<(), LogbookError> {
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
            })
    }
}

#[coverage(off)]
#[cfg(test)]
mod tests;
