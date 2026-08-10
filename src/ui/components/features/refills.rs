mod draft_refill;
mod draft_refill_item;

use draft_refill::DraftRefill;
use draft_refill_item::DraftRefillItem;

// ------------------------------------
use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::domain::{Logbook, LogbookSnapshot, ScriptId, Supply};
use crate::ui::components::DateInput;

#[component]
pub fn Refills() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/refills.css")}
        div {
            class: "refills",
            EligibleSupplies {  }
            PreviousSupplies {  }
        }
    }
}

#[component]
fn EligibleSupplies() -> Element {
    let mut logbook = use_context::<Signal<Logbook>>();
    let snapshot = use_context::<ReadSignal<LogbookSnapshot>>();

    let mut draft = use_signal(DraftRefill::default);
    provide_context(draft);

    use_effect(move || {
        draft
            .write()
            .with_scripts(snapshot.read().eligible_scripts());
    });

    rsx! {
        div {
            class: "refills__eligible-supplies",
            IssuedOn { }
            EligibleSuppliesList { }
            EligibleSuppliesCommands {
                on_submit: move |supplies: Vec<_>| supplies.into_iter().for_each(|s| logbook.write().record_supply_unchecked(s)),
            }
        }
    }
}

#[component]
fn IssuedOn() -> Element {
    let mut draft = use_context::<Signal<DraftRefill>>();

    rsx! {
        DateInput {
            id: "issued-on",
            label: tid!("refill-form-issued-on-label"),
            value: draft.read().issued_on,
            on_change: move |issued_on| draft.write().issued_on = issued_on,
        }
    }
}

#[component]
fn EligibleSuppliesList() -> Element {
    let draft = use_context::<Signal<DraftRefill>>();

    let script_ids = draft.read().items.keys().copied().collect::<Vec<_>>();

    rsx! {
        ul {
            class: "refills__eligible-supplies__list",
            for script_id in script_ids {
                EligibleSuppliesListItem { script_id }
            }
        }
    }
}

#[component]
fn EligibleSuppliesListItem(script_id: ScriptId) -> Element {
    let logbook = use_context::<Signal<Logbook>>();
    let script = logbook.read().script_unchecked(script_id).clone();

    let draft = use_context::<Signal<DraftRefill>>();

    let items = draft
        .read()
        .items
        .get(&script_id)
        .cloned()
        .unwrap_or(Vec::default());

    rsx! {
        li {
            class: "refills__eligible-supplies__list-item",
            {tid!("script-description",
                issued_on: script.issued_on().to_string(),
                expires_on: script.expires_on().to_string())}
            EligibleMedicationsList { items }
        }
    }
}

#[component]
fn EligibleMedicationsList(items: Vec<DraftRefillItem>) -> Element {
    let mut draft = use_context::<Signal<DraftRefill>>();

    rsx! {
        ul {
            class: "refills__eligible-medications__list",
            for item in items {
                EligibleMedicationsListItem {
                    item,
                    on_change: move |item| draft.write().update_item(item),
                }
            }
        }
    }
}

#[component]
fn EligibleMedicationsListItem(
    item: DraftRefillItem,
    on_change: EventHandler<DraftRefillItem>,
) -> Element {
    let logbook = use_context::<Signal<Logbook>>();
    let medication = logbook
        .read()
        .medication_unchecked(item.medication_id)
        .clone();

    let is_selected = item.selected;

    let status = item.status.to_string();

    rsx! {
        li {
            class: "refills__eligible-medications__list-item",
            input {
                r#type: "checkbox",
                checked: is_selected,
                onchange: move |event| {
                    item.selected = event.checked();
                    on_change.call(item)
                }
            }
            span { {tid!("medication-description", name: medication.name(), strength: medication.strength())} }
            span { {tid!(&status)} }
        }
    }
}

#[component]
fn EligibleSuppliesCommands(on_submit: EventHandler<Vec<Supply>>) -> Element {
    let draft = use_context::<Signal<DraftRefill>>();

    rsx! {
        button {
            title: tid!("dispensed-button.hint"),
            aria_label: tid!("dispensed-button.aria-label"),
            disabled: draft.read().selected_items().count() == 0,
            onclick: move |_| {
                let supplies = draft.read().as_supplies().collect::<Vec<_>>();
                on_submit.call(supplies);
            },
            {tid!("dispensed-button.text")}
        }
    }
}

#[component]
fn PreviousSupplies() -> Element {
    rsx! {
        div {
            class: "refills__previous-supplies",
            PreviousSuppliesList {}
        }
    }
}

#[component]
fn PreviousSuppliesList() -> Element {
    let logbook = use_context::<Signal<Logbook>>();

    let mut supplies = logbook.read().supplies().cloned().collect::<Vec<_>>();
    supplies.sort_by_key(|s| s.issued_on());
    supplies.reverse();

    rsx! {
        ul {
            class: "refills__previous-supplies__list",
            for supply in supplies {
                PreviousSuppliesListItem { supply }
            }
        }
    }
}

#[component]
fn PreviousSuppliesListItem(supply: Supply) -> Element {
    let logbook = use_context::<Signal<Logbook>>();

    let medication = logbook
        .read()
        .medication_unchecked(supply.medication_id())
        .clone();
    let issued_on = supply.issued_on().to_string();

    rsx! {
        li {
            class: "refills__previous-supplies__list-item",
            span { {issued_on} }
            span { {tid!("medication-description", name: medication.name(), strength: medication.strength())} }
        }
    }
}
