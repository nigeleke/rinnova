use crate::domain::{
    Date, Logbook, MedicationSnapshot, MedicationStatus, Period, ScriptItemSnapshot,
    ScriptItemStatus, ScriptSnapshot, ScriptStatus, SupplyCount,
};

#[derive(Default)]
pub struct LogbookSnapshot {
    medications: Vec<MedicationSnapshot>,
    scripts: Vec<ScriptSnapshot>,
}

impl LogbookSnapshot {
    pub fn from(logbook: &Logbook, as_of: Date) -> Self {
        let (past_scripts, current_scripts, _future_scripts) = logbook.scripts().fold(
            (Vec::new(), Vec::new(), Vec::new()),
            |(mut past, mut current, mut future), script| {
                if as_of < script.issued_on() {
                    future.push(script);
                } else if as_of > script.expires_on() {
                    past.push(script);
                } else {
                    current.push(script);
                }
                (past, current, future)
            },
        );

        let (supplies_dispensed, _future_supplies): (Vec<_>, Vec<_>) =
            logbook.supplies().partition(|s| s.issued_on() <= as_of);

        let scripts = current_scripts
            .into_iter()
            .map(|script| {
                let script_id = script.id();
                let items = script
                    .items()
                    .filter_map(|item| {
                        let medication_id = item.medication_id();
                        logbook.medication(medication_id).map(|medication| {
                            let dispensed_count = supplies_dispensed
                                .iter()
                                .filter(|s| s.item(script_id, medication_id).is_some())
                                .count();
                            let dispensed_count = SupplyCount::from(dispensed_count);
                            let remaining_supplies =
                                item.authorised_repeats() + SupplyCount::ONE - dispensed_count;
                            let status = match remaining_supplies {
                                SupplyCount::ZERO => ScriptItemStatus::NoRepeats,
                                SupplyCount::ONE => ScriptItemStatus::LastRepeat,
                                _ => ScriptItemStatus::SupplyOk,
                            };
                            ScriptItemSnapshot::new(medication.clone(), remaining_supplies, status)
                        })
                    })
                    .collect::<Vec<_>>();
                let exhausted = items
                    .iter()
                    .all(|i| i.remaining_supplies() == SupplyCount::ZERO);

                let due_to_expire = !script.is_valid_on(as_of + Period::script_expiry_warning());
                let status = if exhausted {
                    ScriptStatus::Exhausted
                } else if due_to_expire {
                    ScriptStatus::DueToExpire
                } else {
                    ScriptStatus::Ok
                };
                ScriptSnapshot::new(script.clone(), status, &items)
            })
            .chain(past_scripts.into_iter().map(|script| {
                let items = script
                    .items()
                    .filter_map(|item| {
                        let medication_id = item.medication_id();
                        logbook.medication(medication_id).map(|medication| {
                            ScriptItemSnapshot::new(
                                medication.clone(),
                                SupplyCount::ZERO,
                                ScriptItemStatus::NoRepeats,
                            )
                        })
                    })
                    .collect::<Vec<_>>();
                ScriptSnapshot::new(script.clone(), ScriptStatus::NotCurrent, &items)
            }))
            .collect::<Vec<_>>();

        let medications = logbook
            .medications()
            .map(|medication| {
                let medication_id = medication.id();
                let remaining_supplies = scripts
                    .iter()
                    .map(|s| {
                        s.items()
                            .iter()
                            .filter(|i| i.medication().id() == medication_id)
                            .map(|i| i.remaining_supplies())
                            .sum()
                    })
                    .sum();
                let status = match remaining_supplies {
                    SupplyCount::ZERO => MedicationStatus::NoRepeats,
                    SupplyCount::ONE => MedicationStatus::LastRepeat,
                    _ => MedicationStatus::Ok,
                };
                MedicationSnapshot::new(medication.clone(), status, remaining_supplies)
            })
            .collect::<Vec<_>>();

        Self {
            medications,
            scripts,
        }
    }

    pub fn medications(&self) -> impl Iterator<Item = &MedicationSnapshot> {
        self.medications.iter()
    }

    pub fn scripts(&self) -> impl Iterator<Item = &ScriptSnapshot> {
        self.scripts.iter()
    }

    pub fn eligible_scripts(&self, as_of: Date) -> impl Iterator<Item = &ScriptSnapshot> {
        self.scripts().filter(move |s| s.is_valid(as_of))
    }
}

#[cfg(test)]
mod tests;
