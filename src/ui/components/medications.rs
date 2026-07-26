mod draft_medication;

use draft_medication::DraftMedication;

use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::domain::Logbook;

#[component]
pub fn Medications() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/medications.css")}
        div {
            class: "medications",
            AddMedication {}
            MedicationList {}
        }
    }
}

#[component]
fn MedicationList() -> Element {
    let mut logbook = use_context::<Signal<Logbook>>();

    let medications = logbook
        .read()
        .medications()
        .iter()
        .map(|m| (m.id(), m.to_string()))
        .collect::<Vec<_>>();
    let zero_medications = medications.is_empty();

    let mut warning = use_signal(String::default);

    rsx! {
        div {
            class: "medications__list",
            for (id, description) in medications {
                span { key: "{id}", "{description}"},
                button {
                    onclick: move |_| {
                        if let Err(error) =logbook.write().try_remove_medication(id) {
                            warning.set(tid!(&error.to_string()));
                        } else {
                            warning.write().clear();
                        }
                    },
                    "-"
                }
            }
        }
        if zero_medications {
            p { {tid!("medications-first-para-01")} }
            p { {tid!("medications-first-para-02")} }
        }
    }
}

#[component]
fn AddMedication() -> Element {
    let mut logbook = use_context::<Signal<Logbook>>();

    let mut draft = use_signal(DraftMedication::default);

    let mut can_add = use_signal(|| false);
    use_effect(move || can_add.set(draft.read().is_valid()));

    let mut warning = use_signal(String::default);

    rsx! {
        form {
            class: "medications__add-form",
            onsubmit: move |event| {
                event.prevent_default();
                if *can_add.read() {
                    let medication = draft().into_medication();
                    let name = medication.to_string();
                    if let Err(error) = logbook.write().try_add_medication(medication) {
                        warning.set(tid!(&error.to_string(), name: name));
                    } else {
                        warning.write().clear();
                    };
                    draft.set(DraftMedication::default());
                }
            },

            label {
                "Medication"
                input {
                    value: &*draft.read().name,
                    onchange: move |e| draft.write().name = e.value()
                }
            }

            label {
                "Strength (optional)"
                input {
                    value: &*draft.read().strength,
                    onchange: move |e| draft.write().strength = e.value()
                }
            }

            label {
                "Notes (optional)"
                textarea {
                    value: &*draft.read().notes,
                    rows: 3,
                    onchange: move |e| draft.write().notes = e.value()
                }
            }

            button {
                r#type: "submit",
                disabled: !*can_add.read(),
                "Add medication"
            }

            p {
                class: "medications__add-form__warning",
                hidden: warning.read().is_empty(),
                {warning}
            }
        }
    }
}
