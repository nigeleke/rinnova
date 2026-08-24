use jiff::Span;
use std::collections::HashMap;

use crate::domain::*;

pub struct Fixture {
    today: Date,
    pub logbook: Logbook,
    medications: HashMap<&'static str, MedicationId>,
    scripts: HashMap<&'static str, ScriptId>,
    supplies: HashMap<&'static str, SupplyId>,
}

impl Fixture {
    pub fn new() -> Self {
        Self {
            today: Date::today(),
            logbook: Default::default(),
            medications: Default::default(),
            scripts: Default::default(),
            supplies: Default::default(),
        }
    }

    pub fn today(&self) -> Date {
        self.today
    }

    pub fn yesterday(&self) -> Date {
        self.today - Period::from(Span::new().days(1))
    }

    pub fn future(&self) -> Date {
        self.today + Period::from(Span::new().months(6))
    }

    pub fn past(&self) -> Date {
        self.yesterday() - Period::from(Span::new().months(6))
    }

    pub fn medication(self, name: &'static str) -> Self {
        self.medication_with(name, "strength", "notes")
    }

    pub fn medication_with(mut self, name: &'static str, strength: &str, notes: &str) -> Self {
        let medication =
            Medication::try_new(name, strength, notes).expect("medication should be created");
        let id = medication.id();
        self.logbook
            .try_add_medication(medication)
            .expect("medication should be added");
        self.medications.insert(name, id);
        self
    }

    pub fn has_medication(&self, id: MedicationId) -> bool {
        self.logbook.medications().any(|m| m.id() == id)
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

    pub fn current_script(mut self, name: &'static str, items: &[(&str, usize)]) -> Self {
        let script = self.build_current_script(items);
        self.script(name, script)
    }

    pub fn build_current_script(&mut self, items: &[(&str, usize)]) -> Script {
        self.build_script(self.today(), self.future(), items)
    }

    pub fn expiring_script(mut self, name: &'static str, items: &[(&str, usize)]) -> Self {
        let script = self.build_expiring_script(items);
        self.script(name, script)
    }

    pub fn build_expiring_script(&mut self, items: &[(&str, usize)]) -> Script {
        self.build_script(self.past(), self.today(), items)
    }

    pub fn expired_script(mut self, name: &'static str, items: &[(&str, usize)]) -> Self {
        let script = self.build_expired_script(items);
        self.script(name, script)
    }

    pub fn build_expired_script(&mut self, items: &[(&str, usize)]) -> Script {
        self.build_script(self.past(), self.yesterday(), items)
    }

    fn script(mut self, name: &'static str, script: Script) -> Self {
        let script_id = script.id();
        self.logbook
            .try_add_script(script)
            .expect("script should be added");
        self.scripts.insert(name, script_id);
        self
    }

    fn build_script(
        &mut self,
        issued_on: Date,
        expires_on: Date,
        items: &[(&str, usize)],
    ) -> Script {
        Script::try_new(issued_on, expires_on, &self.script_items(items))
            .expect("fixture script items must be valid")
    }

    pub fn has_script(&self, id: ScriptId) -> bool {
        self.logbook.scripts().any(|s| s.id() == id)
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

    fn script_items(&self, items: &[(&str, usize)]) -> Vec<ScriptItem> {
        items
            .iter()
            .map(|(name, repeats)| {
                let id = self.medication_id_or_unknown(name);
                ScriptItem::new(id, SupplyCount::from(*repeats))
            })
            .collect()
    }

    pub fn supply(
        mut self,
        name: &'static str,
        script: &str,
        medication: &str,
        issued_on: Date,
    ) -> Self {
        let supply = self.build_supply(issued_on, &[(script, medication)]);
        let supply_id = supply.id();
        self.logbook
            .try_add_supply(supply)
            .expect("supply should be recorded");
        self.supplies.insert(name, supply_id);
        self
    }

    pub fn build_supply(&self, issued_on: Date, items: &[(&str, &str)]) -> Supply {
        let items = items
            .iter()
            .map(|i| {
                let script_id = self.script_id_or_unknown(i.0);
                let medication_id = self.medication_id_or_unknown(i.1);
                SupplyItem::new(script_id, medication_id)
            })
            .collect::<Vec<_>>();

        Supply::try_new(issued_on, &items).expect("supply should be created")
    }

    pub fn has_supply(&self, id: SupplyId) -> bool {
        self.logbook.supplies().any(|s| s.id() == id)
    }
}
