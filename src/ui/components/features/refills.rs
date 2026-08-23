mod draft_refill;
mod draft_refill_item;

use draft_refill::DraftRefill;
use draft_refill_item::DraftRefillItem;

// ------------------------------------
use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::domain::{
    Logbook, LogbookError, LogbookSnapshot, Medication, Script, ScriptItemStatus, Supply,
    SupplyCount, SupplyId, SupplyItem,
};
use crate::ui::components::{
    Confirmation, ConfirmationTheme, DateInput, DeleteButton, Notification,
};

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

    let issued_on = use_memo(move || draft.read().issued_on);

    use_effect(move || {
        draft
            .write()
            .with_scripts(snapshot.read().eligible_scripts(*issued_on.read()));
    });

    rsx! {
        div {
            class: "refills__eligible-supplies",
            IssuedOn { }
            EligibleSuppliesList { }
            EligibleSuppliesCommands {
                on_submit: move |supply| if let Err(error) = logbook.write().try_add_supply(supply) {
                    Notification::logbook_error(&error);
                },
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
    let logbook = use_context::<Signal<Logbook>>();
    let draft = use_context::<Signal<DraftRefill>>();

    let script_ids = draft.read().items.keys().copied().collect::<Vec<_>>();
    let mut scripts = logbook
        .read()
        .scripts()
        .filter(|s| script_ids.contains(&s.id()))
        .cloned()
        .collect::<Vec<_>>();
    scripts.sort_by_key(Script::issued_on);

    rsx! {
        ul {
            class: "refills__eligible-supplies__list",
            for script in scripts {
                EligibleSuppliesListItem { script }
            }
        }
    }
}

#[component]
fn EligibleSuppliesListItem(script: Script) -> Element {
    let draft = use_context::<Signal<DraftRefill>>();

    let items = draft
        .read()
        .items
        .get(&script.id())
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
    let logbook = use_context::<Signal<Logbook>>();
    let mut draft = use_context::<Signal<DraftRefill>>();

    let mut item_medications = items
        .into_iter()
        .filter_map(|item| match logbook.read().medication(item.medication_id) {
            Some(medication) => Some((item, medication.clone())),
            None => {
                let error = LogbookError::InvalidMedication(item.medication_id);
                Notification::logbook_error(&error);
                None
            }
        })
        .collect::<Vec<_>>();
    item_medications.sort_by_key(|a| a.1.to_string());

    rsx! {
        ul {
            class: "refills__eligible-medications__list",
            for item_medication in item_medications {
                EligibleMedicationsListItem {
                    item: item_medication.0,
                    medication: item_medication.1,
                    on_change: move |item| draft.write().update_item(item),
                }
            }
        }
    }
}

#[component]
fn EligibleMedicationsListItem(
    item: DraftRefillItem,
    medication: Medication,
    on_change: EventHandler<DraftRefillItem>,
) -> Element {
    let logbook = use_context::<Signal<Logbook>>();
    let remaining_supplies = match logbook.read().script(item.script_id) {
        Some(script) => script.remaining_supplies(medication.id()),
        None => {
            let error = LogbookError::InvalidScript(item.script_id);
            Notification::logbook_error(&error);
            SupplyCount::ZERO
        }
    };

    let is_selected = item.selected;

    let status = item.status;
    let status = match status {
        ScriptItemStatus::SupplyOk => tid!("remaining-supplies", n: remaining_supplies.to_string()),
        ScriptItemStatus::LastRepeat | ScriptItemStatus::NoRepeats => tid!(&status.to_string()),
    };
    let health = item.status.health();

    rsx! {
        li {
            class: "refills__eligible-medications__list-item",
            class: "{health.to_string()}",
            input {
                r#type: "checkbox",
                checked: is_selected,
                onchange: move |event| {
                    item.selected = event.checked();
                    on_change.call(item)
                }
            }
            span { {medication.to_string()} }
            span { {status} }
        }
    }
}

#[component]
fn EligibleSuppliesCommands(on_submit: EventHandler<Supply>) -> Element {
    let draft = use_context::<Signal<DraftRefill>>();

    rsx! {
        button {
            title: tid!("dispensed-button.hint"),
            aria_label: tid!("dispensed-button.aria-label"),
            disabled: draft.read().selected_items().count() == 0,
            onclick: move |_| {
                let supply = draft.read().as_supply();
                on_submit.call(supply);
            },
            {tid!("dispensed-button.text")}
        }
    }
}

#[component]
fn PreviousSupplies() -> Element {
    let mut logbook = use_context::<Signal<Logbook>>();

    let mut selected_supply_id = use_signal(|| None::<SupplyId>);
    provide_context(selected_supply_id);

    let mut delete_confirmation = use_signal(|| None::<SupplyId>);

    rsx! {
        div {
            class: "refills__previous-supplies",
            PreviousSuppliesScriptsList {}
            PreviousSuppliesCommands {
                on_delete: move |id| delete_confirmation.set(Some(id)),
            }
            if let Some(id) = *delete_confirmation.read()
                && let Some(supply) = logbook.read().supply(id) {
                Confirmation {
                    theme: ConfirmationTheme::Destructive,
                    message: tid!("delete-supply", supply: supply.to_string()),
                    on_ok: move |_| {
                        if let Err(error) = logbook.write().try_remove_supply(id) {
                            Notification::logbook_error(&error);
                        }
                        selected_supply_id.set(None);
                        delete_confirmation.set(None);
                    },
                    on_cancel: move |_| delete_confirmation.set(None),
                }
            }

        }
    }
}

#[component]
fn PreviousSuppliesScriptsList() -> Element {
    let logbook = use_context::<Signal<Logbook>>();

    let mut supplies = logbook.read().supplies().cloned().collect::<Vec<_>>();
    supplies.sort_by_key(|s| s.issued_on());
    supplies.reverse();

    rsx! {
        ul {
            class: "refills__previous-supplies__scripts-list",
            for supply in supplies {
                PreviousSuppliesScriptsListItem { supply }
            }
        }
    }
}

#[component]
fn PreviousSuppliesScriptsListItem(supply: Supply) -> Element {
    let supply_id = supply.id();
    let issued_on = supply.issued_on().to_string();
    let mut selected_supply_id = use_context::<Signal<Option<SupplyId>>>();

    rsx! {
        li {
            class: "refills__previous-supplies__scripts-list-item",
            class: if *selected_supply_id.read() == Some(supply_id) { "selected" },
            key: "{supply_id}",
            onclick: move |_| selected_supply_id.set(Some(supply_id)),
            div { {issued_on} }
            PreviousSuppliesMedicationsList { supply }
        }
    }
}

#[component]
fn PreviousSuppliesMedicationsList(supply: Supply) -> Element {
    let items = Vec::from_iter(supply.items().cloned());

    rsx! {
        ul {
            class: "refills__previous-supplies__medications-list",
            for item in items {
                PreviousSuppliesMedicationsListItem { item }
            }
        }
    }
}

#[component]
fn PreviousSuppliesMedicationsListItem(item: SupplyItem) -> Element {
    let logbook = use_context::<Signal<Logbook>>();

    let medication_id = item.medication_id();
    let medication = match logbook.read().medication(medication_id) {
        Some(medication) => Some(medication.clone()),
        None => {
            let error = LogbookError::InvalidMedication(medication_id);
            Notification::logbook_error(&error);
            None
        }
    };

    let script_id = item.script_id();
    let script = match logbook.read().script(script_id) {
        Some(script) => Some(script.clone()),
        None => {
            let error = LogbookError::InvalidScript(script_id);
            Notification::logbook_error(&error);
            None
        }
    };

    rsx! {
        if let (Some(script), Some(medication)) = (script, medication) {
            li {
                class: "refills__previous-supplies__medications-list-item",
                span { {medication.to_string()} }
                span {
                    {tid!(
                        "script-short-description",
                        issued_on: script.issued_on().to_string()
                    )}
                }
            }
        }
    }
}

#[component]
fn PreviousSuppliesCommands(on_delete: EventHandler<SupplyId>) -> Element {
    let selected_supply_id = use_context::<Signal<Option<SupplyId>>>();

    rsx! {
        div {
            class: "refills__previous-supplies__commands",
            DeleteButton {
                definite_object: tid!("supply-definite"),
                disabled: selected_supply_id.read().is_none(),
                onclick: move |_| selected_supply_id.read().iter().for_each(|id| on_delete.call(*id)),
            }
        }
    }
}
