mod draft_medication;

use draft_medication::DraftMedication;

// ------------------------------------
use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::domain::{Logbook, Medication, MedicationId};
use crate::ui::components::{
    AddButton, CancelButton, Confirmation, ConfirmationTheme, DeleteButton, EditButton, Modal,
    Notification, OkButton,
};

#[component]
pub fn Medications() -> Element {
    let mut logbook = use_context::<Signal<Logbook>>();
    let mut notifications = use_context::<Signal<Vec<Notification>>>();

    let selected_medication_id = use_signal(|| None::<MedicationId>);
    provide_context(selected_medication_id);

    let mut draft = use_signal(|| None::<DraftMedication>);
    let mut delete_confirmation = use_signal(|| None::<MedicationId>);

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/medications.css")}
        div {
            class: "medications",
            MedicationsList { }
            MedicationsCommands {
                on_add: move || draft.set(Some(DraftMedication::default())),
                on_edit: move |id| draft.set(logbook.read().medication(id).map(DraftMedication::from)),
                on_delete: move |id| delete_confirmation.set(Some(id)),
            }

            if let Some(value) = draft() {
                Modal {
                    on_close: move |_| draft.set(None),
                    MedicationForm {
                        value: value,
                        on_submit: move |m| {},
                        on_cancel: move |_| draft.set(None),
                        on_close: move || draft.set(None),
                    }
                }
            }

            if let Some(id) = *delete_confirmation.read()
                && let Some(medication) = logbook.read().medication(id) {
                Confirmation {
                    theme: ConfirmationTheme::Error,
                    message: tid!("delete-medication", medication: medication.to_string()),
                    on_ok: move |_| {
                        if let Err(error) = logbook.write().try_remove_medication(id) {
                            let error = tid!(&error.to_string());
                            notifications.push(Notification::error(&error))
                        }
                        draft.set(None);
                        delete_confirmation.set(None);
                    },
                    on_cancel: move |_| delete_confirmation.set(None),

                }
            }
        }
    }
}

#[component]
fn MedicationsList() -> Element {
    let logbook = use_context::<Signal<Logbook>>();

    let medications = logbook
        .read()
        .medications()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
    let zero_medications = medications.is_empty();

    rsx! {
        ul {
            class: "medications__list",
            for medication in medications {
                MedicationsListItem { medication }
            }
        }
        if zero_medications {
            p { {tid!("zero-medications-para-01")} }
            p { {tid!("zero-medications-para-02")} }
        }
    }
}

#[component]
fn MedicationsListItem(medication: Medication) -> Element {
    let medication_id = medication.id();
    let mut selected_medication_id = use_context::<Signal<Option<MedicationId>>>();

    rsx! {
        li {
            class: "medications__list-item",
            class: if *selected_medication_id.read() == Some(medication_id) { "selected" },
            key: "{medication_id}",
            onclick: move |_| selected_medication_id.set(Some(medication_id)),
            "{medication}"
        }
    }
}

#[component]
fn MedicationsCommands(
    on_add: EventHandler<()>,
    on_edit: EventHandler<MedicationId>,
    on_delete: EventHandler<MedicationId>,
) -> Element {
    let id = use_context::<Signal<Option<MedicationId>>>();

    rsx! {
        div {
            class: "medications__commands",
            AddButton {
                indefinite_object: tid!("medication-indefinite"),
                onclick: move |_| on_add.call(()),
            }
            EditButton {
                definite_object: tid!("medication-definite"),
                disabled: id.read().is_none(),
                onclick: move |_| id.read().iter().for_each(|id| on_edit.call(*id)),
            }
            DeleteButton {
                definite_object: tid!("medication-definite"),
                disabled: id.read().is_none(),
                onclick: move |_| id.read().iter().for_each(|id| on_delete.call(*id)),
            }
        }
    }
}

#[component]
fn MedicationForm(
    value: DraftMedication,
    on_submit: EventHandler<DraftMedication>,
    on_cancel: EventHandler<()>,
    on_close: EventHandler<()>,
) -> Element {
    let mut draft = use_signal(|| value);

    let mut can_submit = use_signal(|| false);
    use_effect(move || can_submit.set(draft.read().is_valid()));

    rsx! {
        form {
            class: "medications__form",
            onsubmit: move |event| {
                event.prevent_default();
                if *can_submit.read() {
                    on_submit.call(draft.read().clone())
                }
            },

            label {
                {tid!("medication-form-medication-label")}
                input {
                    value: &*draft.read().name,
                    onchange: move |e| draft.write().name = e.value()
                }
            }

            label {
                {tid!("medication-form-strength-label")}
                input {
                    value: &*draft.read().strength,
                    onchange: move |e| draft.write().strength = e.value()
                }
            }

            label {
                {tid!("medication-form-notes-label")}
                textarea {
                    value: &*draft.read().notes,
                    rows: 3,
                    onchange: move |e| draft.write().notes = e.value()
                }
            }

            div {
                CancelButton {
                    onclick: move |_| on_cancel.call(())
                }

                OkButton {
                    disabled: !*can_submit.read(),
                    onclick: move |_| on_submit.call(draft.read().cloned())
                }
            }
        }
    }
}
