use std::collections::HashMap;

use jiff::civil::Date;
use jiff::Span;
use serde::{Deserialize, Serialize};

use crate::domain::{
    Health, LogbookError, Medication, MedicationId, Reminder, Script, ScriptId, ScriptItem, Status,
    Supply, SupplyCount,
};

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Logbook {
    medications: Vec<Medication>,
    scripts: Vec<Script>,
    supplies: Vec<Supply>,
}

const DAYS_WARNING: i64 = 14;

impl Logbook {
    pub fn medications(&self) -> &[Medication] {
        &self.medications
    }

    pub fn try_add_medication(&mut self, medication: Medication) -> Result<(), LogbookError> {
        let duplicated =
            |m: &&Medication| m.id() == medication.id() || m.equivalent_to(&medication);

        match self.medications.iter().find(duplicated) {
            Some(m) if m.id() == medication.id() => Err(LogbookError::DuplicateMedication(m.id())),
            Some(m) => Err(LogbookError::MatchingMedication(m.to_string())),
            None => {
                self.medications.push(medication);
                Ok(())
            }
        }
    }

    pub fn try_remove_medication(&mut self, id: MedicationId) -> Result<(), LogbookError> {
        let is_referenced = || {
            self.scripts
                .iter()
                .flat_map(Script::items)
                .any(|item| item.medication_id() == id)
        };

        match self.medications.iter().position(|m| m.id() == id) {
            None => Err(LogbookError::InvalidMedication(id)),
            Some(_) if is_referenced() => Err(LogbookError::MedicationUsedInScript(id)),
            Some(index) => {
                self.medications.remove(index);
                Ok(())
            }
        }
    }

    pub fn scripts(&self) -> &[Script] {
        &self.scripts
    }

    pub fn try_add_script(&mut self, script: Script) -> Result<(), LogbookError> {
        let duplicated = |s: &&Script| s.id() == script.id();
        let duplicate = self.scripts.iter().find(duplicated);

        let valid_medication_ids = self
            .medications
            .iter()
            .map(Medication::id)
            .collect::<Vec<_>>();
        let not_valid = |item: &ScriptItem| !valid_medication_ids.contains(&item.medication_id());

        let invalid_item = script.items().find(not_valid);

        match (duplicate, invalid_item) {
            (Some(s), _) => Err(LogbookError::DuplicateScript(s.id())),
            (_, Some(item)) => Err(LogbookError::UnknownMedication(item.medication_id())),
            (None, None) => {
                self.scripts.push(script);
                Ok(())
            }
        }
    }

    pub fn try_remove_script(&mut self, id: ScriptId) -> Result<(), LogbookError> {
        match self.scripts.iter().position(|s| s.id() == id) {
            Some(index) => {
                self.scripts.remove(index);
                Ok(())
            }
            None => Err(LogbookError::InvalidScript(id)),
        }
    }

    pub fn supplies(&self) -> &[Supply] {
        &self.supplies
    }

    pub fn record_supply(&mut self, supply: Supply) -> Result<(), LogbookError> {
        let supply_id = supply.id();
        (!self.supplies.iter().any(|s| s.id() == supply_id))
            .then_some(())
            .ok_or(LogbookError::DuplicateSupply(supply_id))?;

        let script_id = supply.script_id();
        let script = self
            .scripts
            .iter_mut()
            .find(|s| s.id() == script_id)
            .ok_or(LogbookError::InvalidScript(script_id))?;

        let medication_id = supply.medication_id();
        self.medications
            .iter()
            .find(|m| m.id() == medication_id)
            .ok_or(LogbookError::InvalidMedication(medication_id))?;

        let issued_on = supply.issued_on();
        (issued_on >= script.issued_on() && issued_on <= script.expires_on())
            .then_some(())
            .ok_or(LogbookError::ScriptOutOfDate(script_id))?;

        script
            .items()
            .find(|i| i.medication_id() == medication_id)
            .ok_or(LogbookError::MedicationNotOnScript(
                supply.script_id(),
                supply.medication_id(),
            ))?;

        (self.script_supply_count(script_id, medication_id) > SupplyCount::ZERO)
            .then_some(())
            .ok_or(LogbookError::MedicationOutOfRefills(
                script_id,
                medication_id,
            ))?;

        self.supplies.push(supply);
        Ok(())
    }

    pub fn script_supply_count(
        &self,
        script_id: ScriptId,
        medication_id: MedicationId,
    ) -> SupplyCount {
        self.scripts
            .iter()
            .find(|s| s.id() == script_id)
            .map(|s| self.remaining_supply_count(s, medication_id))
            .unwrap_or(SupplyCount::ZERO)
    }

    pub fn medication_supply_count(&self, medication_id: MedicationId) -> SupplyCount {
        self.scripts
            .iter()
            .map(|s| self.remaining_supply_count(s, medication_id))
            .sum()
    }

    fn remaining_supply_count(&self, script: &Script, medication_id: MedicationId) -> SupplyCount {
        let authorised = script.authorised_supplies(medication_id);

        let supplied = self
            .supplies
            .iter()
            .filter(|s| s.script_id() == script.id() && s.medication_id() == medication_id)
            .count()
            .into();

        authorised - supplied
    }

    pub fn evaluate_status(&self, as_of: Date) -> Vec<Status> {
        self.script_statuses(as_of)
            .chain(self.supply_statuses(as_of))
            .chain(self.coverage_statuses(as_of))
            .collect()
    }

    fn script_statuses(&self, as_of: Date) -> impl Iterator<Item = Status> + '_ {
        self.scripts.iter().map(move |script| {
            let script_id = script.id();
            let expiry_date = script.expires_on();
            let warning_date = expiry_date - Span::new().days(DAYS_WARNING);

            let exhausted = self.is_script_exhausted(script);

            if as_of > expiry_date {
                Status::ScriptExpired(script_id)
            } else if exhausted {
                Status::ScriptExhausted(script_id)
            } else if as_of >= warning_date {
                Status::ScriptDueToExpire(script_id)
            } else {
                Status::ScriptOk(script_id)
            }
        })
    }

    fn is_script_exhausted(&self, script: &Script) -> bool {
        let script_id = script.id();

        // A script is exhausted only when all medications on the script
        // have no remaining supplies. Individual medication exhaustion is
        // reported through NoRepeats(ScriptId, MedicationId).
        script
            .items()
            .all(|i| self.script_supply_count(script_id, i.medication_id()) == SupplyCount::ZERO)
    }

    fn supply_statuses(&self, as_of: Date) -> impl Iterator<Item = Status> + '_ {
        self.usable_scripts(as_of)
            .flat_map(|script| self.script_supply_statuses(script))
    }

    fn usable_scripts(&self, as_of: Date) -> impl Iterator<Item = &Script> + '_ {
        self.scripts
            .iter()
            .filter(move |script| script.is_valid_on(as_of))
    }

    fn script_supply_statuses<'a>(
        &'a self,
        script: &'a Script,
    ) -> impl Iterator<Item = Status> + 'a {
        let script_id = script.id();
        script
            .items()
            .map(move |item| self.script_medication_supply_status(script_id, &item))
    }

    fn script_medication_supply_status(&self, script_id: ScriptId, item: &ScriptItem) -> Status {
        let medication_id = item.medication_id();
        match self.script_supply_count(script_id, medication_id) {
            SupplyCount::ZERO => Status::NoRepeats(script_id, medication_id),
            SupplyCount::ONE => Status::LastRepeat(script_id, medication_id),
            _ => Status::SupplyOk(script_id, medication_id),
        }
    }

    fn coverage_statuses(&self, as_of: Date) -> impl Iterator<Item = Status> + '_ {
        let scripts_by_medication = self.scripts_by_medication(as_of);

        self.medications.iter().map(move |medication| {
            let medication_id = medication.id();

            let valid_with_supply = |scripts: &Vec<&Script>| {
                scripts
                    .iter()
                    .filter(|s| self.script_supply_count(s.id(), medication_id) > SupplyCount::ZERO)
                    .any(|s| s.is_valid_on(as_of))
            };

            let covered = scripts_by_medication
                .get(&medication_id)
                .map(valid_with_supply)
                .unwrap_or(false);

            if covered {
                Status::MedicationOk(medication_id)
            } else {
                Status::MedicationNotCovered(medication_id)
            }
        })
    }

    fn scripts_by_medication(&self, as_of: Date) -> HashMap<MedicationId, Vec<&Script>> {
        let medication_scripts = self
            .medications
            .iter()
            .map(|m| (m.id(), Vec::new()))
            .collect::<HashMap<MedicationId, Vec<&Script>>>();

        let mut medication_scripts =
            self.usable_scripts(as_of)
                .fold(medication_scripts, |mut acc, script| {
                    script.items().for_each(|item| {
                        acc.entry(item.medication_id()).or_default().push(script);
                    });
                    acc
                });

        medication_scripts
            .values_mut()
            .for_each(|value| value.sort_by_key(|s| s.issued_on()));

        medication_scripts
    }

    pub fn script_health(&self, script_id: ScriptId, as_of: Date) -> Health {
        self.evaluate_status(as_of)
            .iter()
            .filter(|status| status.references_script(script_id))
            .filter_map(|status| status.script_health())
            .max()
            .unwrap_or(Health::Ok)
    }

    pub fn supply_health(
        &self,
        script_id: ScriptId,
        medication_id: MedicationId,
        as_of: Date,
    ) -> Health {
        self.evaluate_status(as_of)
            .iter()
            .filter(|status| status.references_supply(script_id, medication_id))
            .filter_map(|status| status.supply_health())
            .max()
            .unwrap_or(Health::Ok)
    }

    pub fn medication_health(&self, medication_id: MedicationId, as_of: Date) -> Health {
        self.evaluate_status(as_of)
            .iter()
            .filter(|status| status.references_medication(medication_id))
            .filter_map(|status| status.medication_health())
            .min()
            .unwrap_or(Health::Critical)
    }

    pub fn reminders_for(&self, _date: Date) -> impl Iterator<Item = &Reminder> {
        std::iter::empty()
    }
}
