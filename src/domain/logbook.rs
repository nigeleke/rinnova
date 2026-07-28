use dioxus::logger::tracing::info;
use jiff::civil::Date;
use serde::{Deserialize, Serialize};

use crate::domain::{
    LogbookError, Medication, MedicationId, Reminder, Script, ScriptId, ScriptItem, Supply,
};

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Logbook {
    medications: Vec<Medication>,
    scripts: Vec<Script>,
    supplies: Vec<Supply>,
}

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

    #[cfg(test)]
    fn add_medication(&mut self, medication: Medication) {
        self.try_add_medication(medication)
            .expect("medication should be added to logbook");
    }

    pub fn try_remove_medication(&mut self, id: MedicationId) -> Result<(), LogbookError> {
        match self.medications.iter().position(|m| m.id() == id) {
            Some(index) => {
                self.medications.remove(index);
                Ok(())
            }
            None => Err(LogbookError::InvalidMedication(id)),
        }
    }

    #[cfg(test)]
    fn remove_medication(&mut self, id: MedicationId) {
        self.try_remove_medication(id)
            .expect("medication should be removed from logbook");
    }

    pub fn try_add_script(&mut self, script: Script) -> Result<(), LogbookError> {
        let duplicated = |s: &&Script| &s.id() == &script.id();
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

    #[cfg(test)]
    fn add_script(&mut self, script: Script) {
        self.try_add_script(script)
            .expect("script should be added to logbook");
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

    #[cfg(test)]
    fn remove_script(&mut self, id: ScriptId) {
        self.try_remove_script(id)
            .expect("script should be removed from logbook");
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

        (self.script_supplies(script_id, medication_id) > 0)
            .then_some(())
            .ok_or(LogbookError::MedicationOutOfRefills(
                script_id,
                medication_id,
            ))?;

        self.supplies.push(supply);
        Ok(())
    }

    pub fn script_supplies(&self, script_id: ScriptId, medication_id: MedicationId) -> usize {
        self.scripts
            .iter()
            .find(|s| s.id() == script_id)
            .map(|s| self.remaining_supplies(s, medication_id))
            .unwrap_or(0)
    }

    pub fn medication_supplies(&self, medication_id: MedicationId) -> usize {
        self.scripts
            .iter()
            .map(|s| self.remaining_supplies(s, medication_id))
            .sum()
    }

    fn remaining_supplies(&self, script: &Script, medication_id: MedicationId) -> usize {
        let authorised = script.authorised_supplies(medication_id);

        let supplied = self
            .supplies
            .iter()
            .filter(|s| s.script_id() == script.id() && s.medication_id() == medication_id)
            .count();

        authorised - supplied
    }

    pub fn reminders_for(&self, _date: Date) -> impl Iterator<Item = &Reminder> {
        std::iter::empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::collections::HashMap;

    use crate::domain::{logbook, Script, ScriptId, Supply, SupplyId};

    #[derive(Default)]
    pub struct Fixture {
        pub logbook: Logbook,
        medications: HashMap<&'static str, MedicationId>,
        scripts: HashMap<&'static str, ScriptId>,
    }

    impl Fixture {
        pub fn today() -> Date {
            jiff::Zoned::now().date()
        }

        pub fn yesterday() -> Date {
            Self::today().yesterday().expect("yesterday existed")
        }

        pub fn future() -> Date {
            Self::today() + jiff::Span::new().months(6)
        }

        pub fn past() -> Date {
            Self::yesterday() - jiff::Span::new().months(6)
        }

        pub fn medication(self, name: &'static str) -> Self {
            self.medication_with(name, "strength", "notes")
        }

        pub fn medication_with(mut self, name: &'static str, strength: &str, notes: &str) -> Self {
            let med = Medication::new(name, strength, notes);
            let id = med.id();
            self.logbook.add_medication(med);
            self.medications.insert(name, id);
            self
        }

        pub fn current_script(mut self, name: &'static str, items: &[(&str, usize)]) -> Self {
            let script = self.build_current_script(items);
            let script_id = script.id();
            self.logbook.add_script(script);
            self.scripts.insert(name, script_id);
            self
        }

        pub fn build_current_script(&mut self, items: &[(&str, usize)]) -> Script {
            Script::try_new(Self::today(), Self::future(), &self.script_items(items))
                .expect("fixture script items must be valid")
        }

        pub fn expired_script(mut self, name: &'static str, items: &[(&str, usize)]) -> Self {
            let script = self.build_expired_script(items);
            let script_id = script.id();
            self.logbook.add_script(script);
            self.scripts.insert(name, script_id);
            self
        }

        pub fn build_expired_script(&mut self, items: &[(&str, usize)]) -> Script {
            Script::try_new(Self::past(), Self::yesterday(), &self.script_items(items))
                .expect("fixture script items must be valid")
        }

        pub fn medication_id(&self, name: &str) -> MedicationId {
            self.medications
                .get(name)
                .copied()
                .unwrap_or_else(|| panic!("fixture medication `{name}` does not exist"))
        }

        pub fn medication_id_or_unknown(&self, name: &str) -> MedicationId {
            self.medications
                .get(name)
                .copied()
                .unwrap_or_else(MedicationId::new)
        }

        pub fn script_id(&self, name: &str) -> ScriptId {
            self.scripts
                .get(name)
                .copied()
                .unwrap_or_else(|| panic!("fixture script `{name}` does not exist"))
        }

        pub fn script_id_or_unknown(&self, name: &str) -> ScriptId {
            self.scripts
                .get(name)
                .copied()
                .unwrap_or_else(ScriptId::new)
        }

        pub fn has_medication(&self, id: MedicationId) -> bool {
            self.logbook.medications.iter().any(|m| m.id() == id)
        }

        pub fn has_script(&self, id: ScriptId) -> bool {
            self.logbook.scripts.iter().any(|s| s.id() == id)
        }

        pub fn has_supply(&self, id: SupplyId) -> bool {
            self.logbook.supplies.iter().any(|s| s.id() == id)
        }

        fn script_items(&self, items: &[(&str, usize)]) -> Vec<ScriptItem> {
            items
                .into_iter()
                .map(|(name, repeats)| {
                    let id = self.medication_id_or_unknown(name);
                    ScriptItem::new(id, *repeats)
                })
                .collect()
        }

        fn build_supply(&self, script: &str, medication: &str, issued_on: Date) -> Supply {
            let script_id = self.script_id_or_unknown(script);
            let medication_id = self.medication_id_or_unknown(medication);
            Supply::new(script_id, medication_id, issued_on)
        }
    }

    #[test]
    fn medication_can_be_added() {
        let fixture = Fixture::default().medication("med01");
        let id = fixture.medication_id("med01");
        assert!(fixture.has_medication(id));
    }

    #[test]
    fn notionally_equivalent_medication_cannot_be_added() {
        let mut fixture = Fixture::default().medication("med01");
        let medication = Medication::new("MED01", "STRENGTH", "others notes");

        let result = fixture.logbook.try_add_medication(medication);
        assert!(matches!(result, Err(LogbookError::MatchingMedication(_))));
    }

    #[test]
    fn existing_medication_can_be_removed() {
        let mut fixture = Fixture::default().medication("med01");
        let id = fixture.medication_id("med01");

        fixture
            .logbook
            .try_remove_medication(id)
            .expect("should remove medication");
        assert!(fixture.logbook.medications.is_empty());
    }

    #[test]
    fn medication_can_be_readded_after_being_removed() {
        let mut fixture = Fixture::default().medication("med01");
        let id = fixture.medication_id("med01");

        fixture.logbook.remove_medication(id);

        let replacement = Medication::new("med01", "strength", "notes");
        let replacement_id = replacement.id();

        fixture
            .logbook
            .try_add_medication(replacement)
            .expect("medication should be added");

        assert!(!fixture.has_medication(id));
        assert!(fixture.has_medication(replacement_id));
    }

    #[test]
    fn non_existing_medication_cannot_be_removed() {
        let mut fixture = Fixture::default().medication("med01");
        let id = fixture.medication_id("med01");

        let unknown = Medication::new("med01", "strength", "notes");
        let unknown_id = unknown.id();

        let result = fixture.logbook.try_remove_medication(unknown_id);
        assert!(matches!(result, Err(LogbookError::InvalidMedication(_))));

        assert!(fixture.has_medication(id));
        assert!(!fixture.has_medication(unknown_id));
    }

    #[test]
    fn script_can_be_added() {
        let mut fixture = Fixture::default().medication("med01");

        let script = fixture.build_current_script(&[("med01", 5)]);
        let script_id = script.id();

        fixture
            .logbook
            .try_add_script(script)
            .expect("script should be added");

        assert!(fixture.has_script(script_id));
    }

    #[test]
    fn script_for_unknown_medication_cannot_be_added() {
        let mut fixture = Fixture::default();

        let unknown_script_item = ScriptItem::new(MedicationId::new(), 2);

        let script = Script::try_new(Fixture::today(), Fixture::future(), &[unknown_script_item])
            .expect("script should be valid");

        let result = fixture.logbook.try_add_script(script);

        assert!(matches!(
            result,
            Err(LogbookError::UnknownMedication(id))
                if id == unknown_script_item.medication_id()
        ));
    }

    #[test]
    fn existing_script_can_be_removed_if_expired() {
        let mut fixture = Fixture::default()
            .medication("med01")
            .expired_script("script01", &[("med01", 5)]);
        let script_id = fixture.script_id("script01");

        fixture
            .logbook
            .try_remove_script(script_id)
            .expect("script should exist");

        assert!(!fixture.has_script(script_id));
    }

    #[test]
    fn existing_script_can_be_removed_if_all_refills_used() {
        panic!("fail");
    }

    #[test]
    fn existing_script_cannot_be_removed_if_current_with_refills() {
        panic!("fail");
    }

    #[test]
    fn existing_script_can_be_removed_if_current_with_refills_if_forced() {
        panic!("fail");
    }

    #[test]
    fn non_existing_script_cannot_be_removed() {
        let mut fixture = Fixture::default()
            .medication("med01")
            .current_script("script01", &[("med01", 5)]);
        let script_id = fixture.script_id("script01");

        let non_existing_id = ScriptId::new();

        let result = fixture.logbook.try_remove_script(non_existing_id);
        assert!(matches!(result, Err(LogbookError::InvalidScript(_))));
        assert!(fixture.has_script(script_id));
    }

    #[test]
    fn medication_not_referenced_by_any_script_can_be_removed() {
        let mut fixture = Fixture::default().medication("unused medication");
        let medication_id = fixture.medication_id("unused medication");

        fixture
            .logbook
            .try_remove_medication(medication_id)
            .expect("medication should be removed");

        assert!(!fixture.has_medication(medication_id));
    }

    #[test]
    fn medication_in_current_script_with_remaining_refills_cannot_be_removed() {
        panic!("fail");
    }

    #[test]
    fn dispensing_can_be_recorded() {
        let mut fixture = Fixture::default()
            .medication("med01")
            .current_script("script01", &[("med01", 5)]);

        let supply = fixture.build_supply("script01", "med01", Fixture::today());
        let supply_id = supply.id();

        fixture
            .logbook
            .record_supply(supply)
            .expect("dispensing should be recorded");

        assert!(fixture.has_supply(supply_id));
    }

    #[test]
    fn dispensing_for_unknown_script_is_rejected() {
        let mut fixture = Fixture::default()
            .medication("med01")
            .current_script("script01", &[("med01", 5)]);

        let supply = fixture.build_supply("unknown", "med01", Fixture::today());
        let supply_id = supply.id();

        let result = fixture.logbook.record_supply(supply);
        assert!(matches!(result, Err(LogbookError::InvalidScript(_))));

        assert!(!fixture.has_supply(supply_id));
    }

    #[test]
    fn dispensing_for_unknown_medication_is_rejected() {
        let mut fixture = Fixture::default()
            .medication("med01")
            .current_script("script01", &[("med01", 5)]);

        let supply = fixture.build_supply("script01", "unknown", Fixture::today());
        let supply_id = supply.id();

        let result = fixture.logbook.record_supply(supply);
        assert!(matches!(result, Err(LogbookError::InvalidMedication(_))));

        assert!(!fixture.has_supply(supply_id));
    }

    #[test]
    fn dispensing_for_medication_not_on_script_is_rejected() {
        let mut fixture = Fixture::default()
            .medication("med01")
            .medication("med02")
            .current_script("script01", &[("med01", 5)]);

        let supply = fixture.build_supply("script01", "med02", Fixture::today());
        let supply_id = supply.id();

        let result = fixture.logbook.record_supply(supply);
        assert!(matches!(
            result,
            Err(LogbookError::MedicationNotOnScript(_, _))
        ));

        assert!(!fixture.has_supply(supply_id));
    }

    #[test]
    fn dispensing_after_script_expiry_is_rejected() {
        let mut fixture = Fixture::default()
            .medication("med01")
            .expired_script("script01", &[("med01", 5)]);

        let supply = fixture.build_supply("script01", "med01", Fixture::today());
        let supply_id = supply.id();

        let result = fixture.logbook.record_supply(supply);
        assert!(matches!(result, Err(LogbookError::ScriptOutOfDate(_))));

        assert!(!fixture.has_supply(supply_id));
    }

    #[test]
    fn dispensing_before_script_issue_date_is_rejected() {
        let mut fixture = Fixture::default()
            .medication("med01")
            .current_script("script01", &[("med01", 5)]);

        let supply = fixture.build_supply("script01", "med01", Fixture::yesterday());
        let supply_id = supply.id();

        let result = fixture.logbook.record_supply(supply);
        assert!(matches!(result, Err(LogbookError::ScriptOutOfDate(_))));

        assert!(!fixture.has_supply(supply_id));
    }

    #[test]
    fn dispensing_can_occur_on_script_issue_date() {
        let mut fixture = Fixture::default()
            .medication("med01")
            .current_script("script01", &[("med01", 5)]);

        let supply = fixture.build_supply("script01", "med01", Fixture::today());
        let supply_id = supply.id();

        fixture
            .logbook
            .record_supply(supply)
            .expect("dispensing should be recorded");

        assert!(fixture.has_supply(supply_id));
    }

    #[test]
    fn dispensing_can_occur_on_script_expiry_date() {
        let mut fixture = Fixture::default()
            .medication("med01")
            .current_script("script01", &[("med01", 5)]);

        let supply = fixture.build_supply("script01", "med01", Fixture::future());
        let supply_id = supply.id();

        fixture
            .logbook
            .record_supply(supply)
            .expect("dispensing should be recorded");

        assert!(fixture.has_supply(supply_id));
    }

    #[test]
    fn dispensing_consumes_one_authorised_repeat() {
        let mut fixture = Fixture::default()
            .medication("med01")
            .current_script("script01", &[("med01", 3)])
            .current_script("script02", &[("med01", 5)]);
        let script_id1 = fixture.script_id("script01");
        let script_id2 = fixture.script_id("script02");
        let medication_id = fixture.medication_id("med01");

        let supply = fixture.build_supply("script01", "med01", Fixture::today());

        let logbook = &mut fixture.logbook;

        assert_eq!(logbook.script_supplies(script_id1, medication_id), 4);
        assert_eq!(logbook.script_supplies(script_id2, medication_id), 6);
        assert_eq!(logbook.medication_supplies(medication_id), 10);

        logbook
            .record_supply(supply)
            .expect("initial supply should be recorded");

        assert_eq!(logbook.script_supplies(script_id1, medication_id), 3);
        assert_eq!(logbook.script_supplies(script_id2, medication_id), 6);
        assert_eq!(logbook.medication_supplies(medication_id), 9);
    }

    #[test]
    fn dispensing_cannot_exceed_authorised_repeats() {
        let mut fixture = Fixture::default()
            .medication("med01")
            .current_script("script01", &[("med01", 0)]);
        let script_id1 = fixture.script_id("script01");
        let medication_id = fixture.medication_id("med01");

        let supply01 = fixture.build_supply("script01", "med01", Fixture::today());
        let supply02 = fixture.build_supply("script01", "med01", Fixture::today());

        let logbook = &mut fixture.logbook;

        assert_eq!(logbook.script_supplies(script_id1, medication_id), 1);
        assert_eq!(logbook.medication_supplies(medication_id), 1);

        logbook
            .record_supply(supply01)
            .expect("initial supply should be recorded");

        assert_eq!(logbook.script_supplies(script_id1, medication_id), 0);
        assert_eq!(logbook.medication_supplies(medication_id), 0);

        let result = logbook.record_supply(supply02);
        assert!(matches!(
            result,
            Err(LogbookError::MedicationOutOfRefills(_, _))
        ))
    }

    #[test]
    fn dispensing_is_allowed_while_authorised_repeats_remain() {
        let mut fixture = Fixture::default()
            .medication("med01")
            .current_script("script01", &[("med01", 4)]);
        let script_id = fixture.script_id("script01");
        let medication_id = fixture.medication_id("med01");

        let supplies = (0..5)
            .map(|_| fixture.build_supply("script01", "med01", Fixture::today()))
            .collect::<Vec<_>>();

        let logbook = &mut fixture.logbook;

        supplies
            .into_iter()
            .try_for_each(|supply| logbook.record_supply(supply))
            .expect("supplies should be recorded");

        assert_eq!(logbook.script_supplies(script_id, medication_id), 0);
        assert_eq!(logbook.medication_supplies(medication_id), 0);
    }

    #[test]
    fn dispensing_history_is_returned_in_chronological_order() {
        panic!("fail");
        // let mut fixture = Fixture::defau()
        //     .medication("med01")
        //     .current_script("script01", &[("med01", 4)]);
        // let script_id = fixture.script_id("script01");
        // let medication_id = fixture.medication_id("med01");

        // let supply01 = fixture.build_supply("script01", "med01", Fixture::today());
        // let supply01_id = supply01.id();

        // let supply02 = fixture.build_supply("script01", "med01", Fixture::today());
        // let supply02_id = supply02.id();

        // let logbook = &mut fixture.logbook;
        // logbook
        //     .record_supply(supply01)
        //     .expect("supply should be recorded");
        // logbook
        //     .record_supply(supply02)
        //     .expect("supply should be recorded");

        // let supplies = logbook.supplies(medication_id);
        // assert_eq!(supplies.next().map(|s| s.medication_id()),)
    }

    #[test]
    fn medication_can_be_dispensed_from_multiple_scripts() {
        let mut fixture = Fixture::default()
            .medication("med01")
            .current_script("script01", &[("med01", 0)])
            .current_script("script02", &[("med01", 0)]);

        let script01_id = fixture.script_id("script01");
        let script02_id = fixture.script_id("script02");
        let medication_id = fixture.medication_id("med01");

        let supply01 = fixture.build_supply("script01", "med01", Fixture::today());
        let supply02 = fixture.build_supply("script02", "med01", Fixture::today());

        let logbook = &mut fixture.logbook;
        logbook
            .record_supply(supply01)
            .expect("supply should be recorded");
        logbook
            .record_supply(supply02)
            .expect("supply should be recorded");

        assert_eq!(logbook.script_supplies(script01_id, medication_id), 0);
        assert_eq!(logbook.script_supplies(script02_id, medication_id), 0);
        assert_eq!(logbook.medication_supplies(medication_id), 0);
    }

    #[test]
    fn dispensing_from_one_script_does_not_affect_remaining_repeats_on_another_script() {
        let mut fixture = Fixture::default()
            .medication("med01")
            .current_script("script01", &[("med01", 0)])
            .current_script("script02", &[("med01", 0)]);

        let script01_id = fixture.script_id("script01");
        let script02_id = fixture.script_id("script02");
        let medication_id = fixture.medication_id("med01");

        let supply01 = fixture.build_supply("script01", "med01", Fixture::today());

        let logbook = &mut fixture.logbook;
        logbook
            .record_supply(supply01)
            .expect("supply should be recorded");

        assert_eq!(logbook.script_supplies(script01_id, medication_id), 0);
        assert_eq!(logbook.script_supplies(script02_id, medication_id), 1);
        assert_eq!(logbook.medication_supplies(medication_id), 1);
    }
}
