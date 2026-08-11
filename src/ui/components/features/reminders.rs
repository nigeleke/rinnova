use std::collections::HashSet;

use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::domain::{LogbookSnapshot, MedicationSnapshot, MedicationStatus, ScriptStatus};

#[component]
pub fn Reminders() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/reminders.css")}
        div {
            class: "reminders",
            ReminderList { }
        }
    }
}

#[component]
fn ReminderList() -> Element {
    let snapshot = use_context::<ReadSignal<LogbookSnapshot>>();
    let snapshot = snapshot.read();

    let medications = snapshot.medications().cloned().collect::<Vec<_>>();
    let scripts = snapshot.scripts().cloned().collect::<Vec<_>>();

    let (no_repeats, rest): (Vec<_>, Vec<_>) = medications
        .into_iter()
        .partition(|m| m.status() == MedicationStatus::NoRepeats);

    let (last_repeats, others): (Vec<_>, Vec<_>) = rest
        .into_iter()
        .partition(|m| m.status() == MedicationStatus::LastRepeat);

    let ok_medication_ids = scripts
        .iter()
        .filter(|s| s.status() == ScriptStatus::Ok)
        .flat_map(|s| s.items().iter().map(|i| i.medication().id()))
        .collect::<HashSet<_>>();

    let due_medication_ids = scripts
        .iter()
        .filter(|s| s.status() == ScriptStatus::DueToExpire)
        .flat_map(|s| s.items().iter().map(|i| i.medication().id()))
        .collect::<HashSet<_>>();

    let script_expiring = others
        .into_iter()
        .filter(|m: &MedicationSnapshot| {
            let id = m.medication().id();
            due_medication_ids.contains(&id) && !ok_medication_ids.contains(&id)
        })
        .collect::<Vec<_>>();

    rsx! {
        ul {
            MedicationsListItems { title: tid!("reminders-subtitle.no-repeats"), medications: no_repeats }
            MedicationsListItems { title: tid!("reminders-subtitle.last-repeats"), medications: last_repeats }
            MedicationsListItems { title: tid!("reminders-subtitle.script-expiring"), medications: script_expiring }
        }
    }
}

#[component]
fn MedicationsListItems(title: String, medications: Vec<MedicationSnapshot>) -> Element {
    rsx! {
        li { h3 { {title.to_string()} } }
        for medication in medications {
            li { {tid!("medication-description", name: medication.name(), strength: medication.strength())} }
        }
    }
}
