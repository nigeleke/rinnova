use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::domain::{LogbookError, Medication, MedicationId, Script, ScriptId, Supply, SupplyId};

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

    pub fn medication_unchecked(&self, id: MedicationId) -> &Medication {
        self.medication(id).expect("{id} must exist")
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
            self.scripts
                .values()
                .flat_map(Script::items)
                .any(|item| item.medication_id() == id)
        };

        match self.medications.get(&id) {
            None => Err(LogbookError::InvalidMedication(id)),
            Some(_) if is_referenced() => Err(LogbookError::MedicationUsedInScript(id)),
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

    pub fn script_unchecked(&self, id: ScriptId) -> &Script {
        self.script(id).expect("{id} must exist")
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
        match self.scripts.remove(&id) {
            Some(_) => Ok(()),
            None => Err(LogbookError::InvalidScript(id)),
        }
    }

    pub fn supplies(&self) -> impl Iterator<Item = &Supply> {
        self.supplies.values()
    }

    pub fn record_supply(&mut self, supply: Supply) -> Result<(), LogbookError> {
        let supply_id = supply.id();
        if self.supplies.contains_key(&supply_id) {
            return Err(LogbookError::DuplicateSupply(supply_id));
        }

        let script_id = supply.script_id();
        let script = self
            .scripts
            .get_mut(&script_id)
            .ok_or(LogbookError::InvalidScript(script_id))?;

        let medication_id = supply.medication_id();
        if !self.medications.contains_key(&medication_id) {
            return Err(LogbookError::InvalidMedication(medication_id));
        }

        let issued_on = supply.issued_on();
        if !script.is_valid_on(issued_on) {
            return Err(LogbookError::ScriptOutOfDate(script_id));
        }

        script
            .items()
            .find(|i| i.medication_id() == medication_id)
            .ok_or(LogbookError::MedicationNotOnScript(
                script_id,
                medication_id,
            ))?;

        self.supplies.insert(supply_id, supply);
        Ok(())
    }

    pub fn record_supply_unchecked(&mut self, supply: Supply) {
        self.record_supply(supply)
            .expect("supply should have been recorded");
    }
}

#[cfg(test)]
mod tests;
