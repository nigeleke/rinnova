use jiff::civil::Date;
use serde::{Deserialize, Serialize};

use crate::domain::{LogbookError, Medication, MedicationId, Reminder};

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Logbook {
    medications: Vec<Medication>,
}

impl Logbook {
    pub fn medications(&self) -> &[Medication] {
        &self.medications
    }

    pub fn add_medication(&mut self, medication: Medication) -> Result<(), LogbookError> {
        match self
            .medications
            .iter()
            .find(|m| m.equivalent_to(&medication))
        {
            Some(m) => Err(LogbookError::DuplicateMedication(m.to_string())),
            None => {
                self.medications.push(medication);
                Ok(())
            }
        }
    }

    pub fn remove_medication(&mut self, id: MedicationId) {
        self.medications.retain(|m| m.id() != id);
    }

    pub fn reminders_for(&self, _date: Date) -> impl Iterator<Item = &Reminder> {
        std::iter::empty()
    }

    // pub fn prescriptions(&self) -> &[Prescription];
    // pub fn refills(&self) -> &[Refill];
    // pub fn reminders_for(&self, date: Data) ->
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn medication_can_be_added() {
        let mut logbook = Logbook::default();
        let medication = Medication::new("name".into(), "strength".into(), "notes".into());

        logbook
            .add_medication(medication.clone())
            .expect("medication should be added");

        assert!(logbook.medications.contains(&medication))
    }

    #[test]
    fn notioanally_equivalent_medication_cannot_be_added() {
        let mut logbook = Logbook::default();
        let medication1 = Medication::new("name".into(), "strength".into(), "notes".into());
        let medication2 = Medication::new("NAME".into(), "STRENGTH".into(), "others notes".into());

        logbook
            .add_medication(medication1.clone())
            .expect("medication should be added");

        let result = logbook.add_medication(medication2);
        assert!(matches!(result, Err(LogbookError::DuplicateMedication(_))));
    }

    #[test]
    fn existing_medication_can_be_removed() {
        let mut logbook = Logbook::default();
        let medication = Medication::new("name".into(), "strength".into(), "notes".into());
        let id = medication.id();

        logbook
            .add_medication(medication.clone())
            .expect("medication should be added");

        logbook.remove_medication(id);

        assert!(logbook.medications.is_empty());
    }

    #[test]
    fn non_existing_medication_will_not_be_removed() {
        let mut logbook = Logbook::default();

        let medication1 = Medication::new("name".into(), "strength".into(), "notes".into());
        let id1 = medication1.id();
        logbook
            .add_medication(medication1.clone())
            .expect("medication should be added");

        let medication2 = Medication::new("name".into(), "strength".into(), "notes".into());
        let id2 = medication2.id();

        logbook.remove_medication(id2);

        assert!(logbook
            .medications
            .iter()
            .map(Medication::id)
            .collect::<Vec<_>>()
            .contains(&id1));
    }

    #[test]
    fn medication_in_active_script_will_not_be_removed() {
        panic!("fail");
    }

    #[test]
    fn medication_in_active_refill_will_n0t_be_removed() {
        panic!("fail");
    }
}
