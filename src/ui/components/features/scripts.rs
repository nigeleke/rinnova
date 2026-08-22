mod draft_script;
mod draft_script_item;

use draft_script::DraftScript;
use draft_script_item::DraftScriptItem;

// --------------------------------------
use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::domain::{
    Logbook, LogbookError, LogbookSnapshot, ScriptId, ScriptItemSnapshot, ScriptItemStatus,
    ScriptSnapshot, ScriptStatus, SupplyCount,
};
use crate::ui::components::{
    AddButton, CancelButton, Confirmation, ConfirmationTheme, DateInput, DeleteButton, EditButton,
    Modal, Notification, OkButton,
};

#[component]
pub fn Scripts() -> Element {
    let mut logbook = use_context::<Signal<Logbook>>();
    let default_draft_script = move || DraftScript::new(logbook.read().medications());

    let mut selected_script_id = use_signal(|| None::<ScriptId>);
    provide_context(selected_script_id);

    let mut draft = use_signal(|| None::<DraftScript>);
    let mut delete_confirmation = use_signal(|| None::<ScriptId>);

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/scripts.css")}
        div {
            class: "scripts",
            ScriptsList { }
            ScriptsCommands {
                on_add: move || draft.set(Some(default_draft_script())),
                on_edit: move |id| {
                    match logbook.read().script(id) {
                        Some(script) => {
                            let update = default_draft_script().using_script(script);
                            draft.set(Some(update));
                        }
                        None => {
                            let error = LogbookError::InvalidScript(id);
                            Notification::internal_error(&error);
                        }
                    }
                },
                on_delete: move |id| delete_confirmation.set(Some(id)),
            }

            if let Some(value) = draft() {
                Modal {
                    on_close: move |_| draft.set(None),
                    ScriptsForm {
                        value,
                        on_submit: move |s: DraftScript| {
                            let id = s.id;
                            if let Err(error) = s.try_into_script().and_then(|script| {
                                match id {
                                    Some(_) => logbook.write().try_update_script(script),
                                    None => logbook.write().try_add_script(script),
                                }
                            }) {
                                Notification::notify(&error);
                            }
                            draft.set(None);
                        },
                        on_cancel: move |_| draft.set(None),
                    }
                }
            }

            if let Some(id) = *delete_confirmation.read()
                && let Some(script) = logbook.read().script(id) {
                Confirmation {
                    theme: ConfirmationTheme::Destructive,
                    message: tid!("delete-script", script: script.to_string()),
                    on_ok: move |_| {
                        if let Err(error) = logbook.write().try_remove_script(id) {
                            Notification::notify(&error);
                        }
                        draft.set(None);
                        selected_script_id.set(None);
                        delete_confirmation.set(None);
                    },
                    on_cancel: move |_| delete_confirmation.set(None),
                }
            }
        }

    }
}

#[component]
fn ScriptsList() -> Element {
    let snapshot = use_context::<ReadSignal<LogbookSnapshot>>();

    let mut scripts = snapshot.read().scripts().cloned().collect::<Vec<_>>();
    scripts.sort_by_key(|s| s.issued_on());

    let zero_scripts = scripts.is_empty();

    rsx! {
       if zero_scripts {
           p { {tid!("zero-scripts-para-01")} }
           p { {tid!("zero-scripts-para-02")} }
       } else {
           ul {
              class: "scripts__list",
              for script in scripts {
                  ScriptsListItem { script }
              }
          }
       }
    }
}

#[component]
fn ScriptsListItem(script: ScriptSnapshot) -> Element {
    let script_id = script.id();
    let health = script.health();
    let status = script.status();
    let mut selected_script_id = use_context::<Signal<Option<ScriptId>>>();

    rsx! {
        li {
            class: "scripts__list-item",
            class: "{health}",
            class: if *selected_script_id.read() == Some(script_id) { "selected" },
            key: "{script_id}",
            onclick: move |_| selected_script_id.set(Some(script_id)),
            div { {tid!("script-description",
                issued_on: script.issued_on().to_string(),
                expires_on: script.expires_on().to_string())} }
            if status != ScriptStatus::Ok {
                div { {tid!(&status.to_string())} }
            }
            MedicationsList { script }
        }

    }
}

#[component]
fn MedicationsList(script: ScriptSnapshot) -> Element {
    let items = Vec::from(script.items());

    rsx! {
        ul {
            class: "scripts__medications__list",
            for item in items {
                MedicationsListItem { item }
            }
        }
    }
}

#[component]
fn MedicationsListItem(item: ScriptItemSnapshot) -> Element {
    let medication_id = item.medication_id();
    let health = item.health();
    let status = item.status();
    let medication = item.medication();
    let remaining_supplies = item.remaining_supplies();
    let status = match status {
        ScriptItemStatus::SupplyOk => tid!("remaining-supplies", n: remaining_supplies.to_string()),
        ScriptItemStatus::LastRepeat | ScriptItemStatus::NoRepeats => tid!(&status.to_string()),
    };

    rsx! {
        li {
            class: "scripts__medications__list-item",
            class: "{health.to_string()}",
            key: "{medication_id}",
            span { {medication.to_string()} }
            span { {status} }
        }
    }
}

#[component]
fn ScriptsCommands(
    on_add: EventHandler<()>,
    on_edit: EventHandler<ScriptId>,
    on_delete: EventHandler<ScriptId>,
) -> Element {
    let logbook = use_context::<Signal<Logbook>>();
    let id = use_context::<Signal<Option<ScriptId>>>();

    rsx! {
        div {
            class: "scripts__commands",
            AddButton {
                indefinite_object: tid!("script-indefinite"),
                onclick: move |_| on_add.call(()),
            }
            EditButton {
                definite_object: tid!("script-definite"),
                disabled: id.read().is_none_or(|id| logbook.read().is_script_immutable(id)),
                onclick: move |_| if let Some(id) = *id.read() { on_edit.call(id) },
            }
            DeleteButton {
                definite_object: tid!("script-definite"),
                disabled: id.read().is_none_or(|id| logbook.read().is_script_immutable(id)),
                onclick: move |_| if let Some(id) = *id.read() { on_delete.call(id) },
            }
        }
    }
}

#[component]
fn ScriptsForm(
    value: DraftScript,
    on_submit: EventHandler<DraftScript>,
    on_cancel: EventHandler<()>,
) -> Element {
    let mut draft = use_signal(|| value);
    provide_context(draft);

    let mut can_submit = use_signal(|| false);
    use_effect(move || can_submit.set(draft.read().is_valid()));

    rsx! {
        form {
            class: "scripts__form",
            onsubmit: move |event| {
                event.prevent_default();
                if *can_submit.read() {
                    on_submit.call(draft.read().clone())
                }
            },

            DateInput {
                id: "issued-on",
                label: tid!("scripts-form-issued-on-label"),
                value: draft.read().issued_on,
                on_change: move |date| draft.write().issued_on = date,
            }

            DateInput {
                id: "expires-on",
                label: tid!("scripts-form-expires-on-label"),
                value: draft.read().expires_on,
                on_change: move |date| draft.write().expires_on = date,
            }

            ScriptItemsList { }

            div {
                class: "scripts__commands",
                CancelButton {
                    onclick: move |_| on_cancel.call(())
                }

                OkButton {
                    disabled: !*can_submit.read(),
                }
            }
        }
    }
}

#[component]
fn ScriptItemsList() -> Element {
    let mut draft = use_context::<Signal<DraftScript>>();

    rsx! {
        ul {
            class: "scripts__script-items__list",
            span { {tid!("scripts-form-medication-heading")} }
            span { {tid!("scripts-form-repeats-heading")} }
            for item in draft().items {
                ScriptItemsListItem {
                    item,
                    on_change: move |item| draft.write().update_item(item),
                }
            }
        }
    }
}

#[component]
fn ScriptItemsListItem(item: DraftScriptItem, on_change: EventHandler<DraftScriptItem>) -> Element {
    let mut item = use_signal(|| item);

    rsx! {
        li {
            class: "scripts__script-items__list-item",

            label {
                class: "scripts__script-items__medication",
                input {
                    r#type: "checkbox",
                    checked: item.read().selected,
                    onchange: move |event| {
                        item.write().selected = event.checked();
                        on_change.call(item());
                    }
                }
                {item.read().medication.to_string()}
            }

            input {
                id: "{item.read().medication.id()}",
                r#type: "number",
                min: "0",
                value: "{item.read().repeats}",
                onchange: move |event| {
                    let count = event.value().parse::<usize>().unwrap_or_default();
                    item.write().selected = count != 0;
                    item.write().repeats = SupplyCount::from(count);
                    on_change.call(item());
                },
            }
        }
    }
}
