mod draft_refill;
mod draft_refill_item;

use draft_refill::DraftRefill;
use draft_refill_item::DraftRefillItem;

// ------------------------------------
use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::domain::{Logbook, LogbookSnapshot, ScriptId, Supply, SupplyId, SupplyItem};
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
                on_submit: move |supply| logbook.write().record_supply_unchecked(supply),
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
            span { {medication.to_string()} }
            span { {tid!(&status)} }
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
                            Notification::notify(error);
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

    let script = logbook.read().script_unchecked(item.script_id()).clone();
    let medication = logbook
        .read()
        .medication_unchecked(item.medication_id())
        .clone();

    rsx! {
        li {
            class: "refills__previous-supplies__medications-list-item",
            span { {medication.to_string()} }
            span { {tid!("script-short-description", issued_on: script.issued_on().to_string())} }
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
