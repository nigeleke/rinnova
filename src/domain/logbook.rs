use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::domain::{
    Date, LogbookError, Medication, MedicationId, Period, Script, ScriptId, Supply, SupplyId,
};

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Logbook {
    medications: HashMap<MedicationId, Medication>,
    scripts: HashMap<ScriptId, Script>,
    supplies: HashMap<SupplyId, Supply>,
}

impl Logbook {
    pub fn medications(&self) -> impl Iterator<Item = &Medication> {
        self.medications.values()
    }

    pub fn medication(&self, id: MedicationId) -> Option<&Medication> {
        self.medications.get(&id)
    }

    pub fn is_medication_immutable(&self, id: MedicationId) -> bool {
        self.scripts
            .values()
            .any(|s| s.items().any(|i| i.medication_id() == id))
    }

    pub fn try_add_medication(&mut self, medication: Medication) -> Result<(), LogbookError> {
        let id = medication.id();
        match self.medications.get(&id) {
            Some(m) => Err(LogbookError::DuplicateMedication(m.id())),
            None => {
                if let Some(m) = self
                    .medications
                    .values()
                    .find(|m| m.equivalent_to(&medication))
                {
                    Err(LogbookError::MatchingMedication(String::from(m.name())))
                } else {
                    self.medications.insert(id, medication);
                    Ok(())
                }
            }
        }
    }

    pub fn try_update_medication(&mut self, medication: Medication) -> Result<(), LogbookError> {
        let medication_id = medication.id();

        if let Some(existing) = self.medications.get_mut(&medication_id) {
            *existing = medication;
            Ok(())
        } else {
            Err(LogbookError::InvalidMedication(medication_id))
        }
    }

    pub fn try_remove_medication(&mut self, id: MedicationId) -> Result<(), LogbookError> {
        let is_referenced = || {
            let script_reference = self
                .scripts
                .values()
                .flat_map(Script::items)
                .any(|item| item.medication_id() == id);
            let supply_reference = self
                .supplies
                .values()
                .flat_map(Supply::items)
                .any(|item| item.medication_id() == id);
            script_reference || supply_reference
        };

        match self.medications.get(&id) {
            None => Err(LogbookError::InvalidMedication(id)),
            Some(_) if is_referenced() => Err(LogbookError::MedicationUsedInScript),
            _ => {
                self.medications.remove(&id);
                Ok(())
            }
        }
    }

    pub fn scripts(&self) -> impl Iterator<Item = &Script> {
        self.scripts.values()
    }

    pub fn script(&self, id: ScriptId) -> Option<&Script> {
        self.scripts.get(&id)
    }

    pub fn is_script_immutable(&self, id: ScriptId) -> bool {
        self.supplies
            .values()
            .any(|s| s.items().any(|i| i.script_id() == id))
    }

    pub fn try_add_script(&mut self, script: Script) -> Result<(), LogbookError> {
        let id = script.id();
        match self.scripts.get(&id) {
            Some(s) => Err(LogbookError::DuplicateScript(s.id())),
            None => {
                let unknown_medication = script
                    .items()
                    .find(|i| self.medication(i.medication_id()).is_none());

                if let Some(item) = unknown_medication {
                    Err(LogbookError::UnknownMedication(item.medication_id()))
                } else {
                    self.scripts.insert(id, script);
                    Ok(())
                }
            }
        }
    }

    pub fn try_update_script(&mut self, script: Script) -> Result<(), LogbookError> {
        let script_id = script.id();

        if let Some(existing) = self.scripts.get_mut(&script_id) {
            *existing = script;
            Ok(())
        } else {
            Err(LogbookError::InvalidScript(script_id))
        }
    }

    pub fn try_remove_script(&mut self, id: ScriptId) -> Result<(), LogbookError> {
        let is_referenced = || {
            self.supplies
                .values()
                .flat_map(Supply::items)
                .any(|item| item.script_id() == id)
        };

        match self.scripts.get(&id) {
            None => Err(LogbookError::InvalidScript(id)),
            Some(_) if is_referenced() => Err(LogbookError::ScriptUsedInSupply),
            Some(_) => {
                self.scripts.remove(&id);
                Ok(())
            }
        }
    }

    pub fn supplies(&self) -> impl Iterator<Item = &Supply> {
        self.supplies.values()
    }

    pub fn supply(&self, supply_id: SupplyId) -> Option<&Supply> {
        self.supplies.get(&supply_id)
    }

    pub fn try_add_supply(&mut self, supply: Supply) -> Result<(), LogbookError> {
        let supply_id = supply.id();
        match self.supplies.get(&supply_id) {
            Some(_) => Err(LogbookError::DuplicateSupply(supply_id)),
            None => self.validate_and_add_supply(supply),
        }
    }

    fn validate_and_add_supply(&mut self, supply: Supply) -> Result<(), LogbookError> {
        let supply_id = supply.id();
        let issued_on = supply.issued_on();

        supply.items().try_for_each(|i| {
            let script_id = i.script_id();
            let medication_id = i.medication_id();

            let valid_script = self.require_script(script_id)?;

            valid_script
                .is_valid_on(issued_on)
                .ok_or(LogbookError::ScriptOutOfDate(script_id))?;

            self.require_medication(medication_id)?;

            valid_script
                .item(medication_id)
                .ok_or(LogbookError::MedicationNotOnScript(
                    script_id,
                    medication_id,
                ))?;

            Ok::<_, LogbookError>(())
        })?;

        self.supplies.insert(supply_id, supply);
        Ok(())
    }

    fn require_medication(&self, id: MedicationId) -> Result<&Medication, LogbookError> {
        self.medication(id)
            .ok_or(LogbookError::InvalidMedication(id))
    }

    fn require_script(&self, id: ScriptId) -> Result<&Script, LogbookError> {
        self.script(id).ok_or(LogbookError::InvalidScript(id))
    }

    pub fn add_supply(&mut self, supply: Supply) {
        self.try_add_supply(supply)
            .expect("supply should have been recorded");
    }

    pub fn try_remove_supply(&mut self, id: SupplyId) -> Result<(), LogbookError> {
        match self.supplies.remove(&id) {
            Some(_) => Ok(()),
            None => Err(LogbookError::InvalidSupply(id)),
        }
    }

    pub fn housekeeping(&mut self, as_of: Date) {
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

        let _ = old_supply_ids
            .iter()
            .try_for_each(|id| self.try_remove_supply(*id))
            .and_then(|_| {
                old_script_ids
                    .iter()
                    .try_for_each(|id| self.try_remove_script(*id))
            });
    }
}

#[coverage(off)]
#[cfg(test)]
mod tests;
