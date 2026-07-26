use jiff::civil::Date;
use serde::{Deserialize, Serialize};

use crate::domain::{
    LogbookError, Medication, MedicationId, Reminder, Script, ScriptId, ScriptItem,
};

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Logbook {
    medications: Vec<Medication>,
    scripts: Vec<Script>,
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

    pub fn reminders_for(&self, _date: Date) -> impl Iterator<Item = &Reminder> {
        std::iter::empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::domain::{Script, ScriptId};

    use std::collections::HashMap;

    struct ScriptBuilder {
        items: Vec<ScriptItem>,
    }

    impl ScriptBuilder {
        pub fn with(mut self, id: MedicationId, authorised_repeats: u8) -> Self {
            let item = ScriptItem::new(id, authorised_repeats);
            self.items.push(item);
            self
        }

        pub fn as_current(self) -> Script {
            self.build_script(Fixture::today(), Fixture::future_date())
        }

        pub fn as_expired(self) -> Script {
            self.build_script(Fixture::past_date(), Fixture::yesterday())
        }

        fn build_script(self, issued_on: Date, expires_on: Date) -> Script {
            Script::try_new(issued_on, expires_on, &self.items).expect("valid script")
        }
    }

    #[derive(Default)]
    struct Fixture {
        pub logbook: Logbook,
        medications: HashMap<String, MedicationId>,
        scripts: HashMap<String, ScriptId>,
    }

    impl Fixture {
        fn today() -> Date {
            jiff::Zoned::now().date()
        }

        fn yesterday() -> Date {
            jiff::Zoned::now()
                .date()
                .yesterday()
                .expect("yesterday existed")
        }

        fn future_date() -> Date {
            Fixture::today() + jiff::Span::new().months(6)
        }

        fn past_date() -> Date {
            Fixture::yesterday() - jiff::Span::new().months(6)
        }

        fn medication(mut self, name: &str) -> Self {
            let medication = Medication::new(name, "strength", "notes");
            let id = medication.id();

            self.logbook.add_medication(medication);

            self.medications.insert(name.into(), id);
            self
        }

        fn medication_id(&self, name: &str) -> MedicationId {
            *self
                .medications
                .get(name)
                .expect("fixture medication does not exist")
        }

        fn has_medication(&self, id: MedicationId) -> bool {
            self.logbook.medications.iter().any(|m| m.id() == id)
        }

        fn script_item(&self, name: &str, authorised_repeats: u8) -> ScriptItem {
            let medication_id = self.medication_id(name);
            ScriptItem::new(medication_id, authorised_repeats)
        }

        fn current_script(mut self, name: &str, builder: ScriptBuilder) -> Self {
            let script = builder.as_current();
            let id = script.id();

            self.logbook.add_script(script);

            self.scripts.insert(name.into(), id);
            self
        }

        fn script_id(&self, name: &str) -> ScriptId {
            *self
                .scripts
                .get(name)
                .expect("fixture script does not exist")
        }

        fn has_script(&self, id: ScriptId) -> bool {
            self.logbook.scripts.iter().any(|s| s.id() == id)
        }
    }

    #[test]
    fn medication_can_be_added() {
        let fixture = Fixture::default().medication("name");
        let id = fixture.medication_id("name");
        assert!(fixture.has_medication(id));
    }

    #[test]
    fn notionally_equivalent_medication_cannot_be_added() {
        let mut fixture = Fixture::default().medication("name");
        let medication = Medication::new("NAME", "STRENGTH", "others notes");

        let result = fixture.logbook.try_add_medication(medication);
        assert!(matches!(result, Err(LogbookError::MatchingMedication(_))));
    }

    #[test]
    fn existing_medication_can_be_removed() {
        let mut fixture = Fixture::default().medication("name");
        let id = fixture.medication_id("name");

        fixture
            .logbook
            .try_remove_medication(id)
            .expect("should remove medication");
        assert!(fixture.logbook.medications.is_empty());
    }

    #[test]
    fn medication_can_be_readded_after_being_removed() {
        let mut fixture = Fixture::default().medication("name");
        let id = fixture.medication_id("name");

        fixture.logbook.remove_medication(id);

        let replacement = Medication::new("name", "strength", "notes");
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
        let mut fixture = Fixture::default().medication("name");
        let id = fixture.medication_id("name");

        let unknown = Medication::new("name", "strength", "notes");
        let unknown_id = unknown.id();

        let result = fixture.logbook.try_remove_medication(unknown_id);
        assert!(matches!(
            result,
            Err(LogbookError::InvalidMedication(unknown_id))
        ));

        assert!(fixture.has_medication(id));
        assert!(!fixture.has_medication(unknown_id));
    }

    #[test]
    fn script_can_be_added() {
        let mut fixture = Fixture::default().medication("name");
        let item = fixture.script_item("name", 2);

        let script = Script::try_new(
            Date::new(2026, 1, 1).expect("date should be valid"),
            Date::new(2027, 1, 1).expect("date should be valid"),
            &[item],
        )
        .expect("script should be valid");

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

        let script = Script::try_new(
            Fixture::today(),
            Fixture::future_date(),
            &[unknown_script_item],
        )
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
        let mut fixture = Fixture::default().medication("name");
        let item = fixture.script_item("name", 5);

        let script = Script::try_new(Fixture::past_date(), Fixture::yesterday(), &[item])
            .expect("script should be valid");

        let script_id = script.id();
        fixture
            .logbook
            .try_add_script(script)
            .expect("script should be added");

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
        let mut fixture = Fixture::default().medication("name");
        let item = fixture.script_item("name", 5);

        let script = Script::try_new(Fixture::today(), Fixture::future_date(), &[item])
            .expect("script should be valid");
        let script_id = script.id();

        fixture
            .logbook
            .try_add_script(script)
            .expect("script should be added");

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
}
