use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::domain::{Date, Logbook, LogbookSnapshot, ScriptId, Supply};

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
    let logbook = use_context::<Signal<Logbook>>();
    let snapshot = use_context::<ReadSignal<LogbookSnapshot>>();
    // provide_context(draft);

    // use_effect(move || {
    //     draft.set(DraftRefill::from(
    //         logbook
    //             .read()
    //             .eligible_supplies(Date::today())
    //             .collect::<Vec<_>>()
    //             .as_slice(),
    //     ));
    // });

    rsx! {
        div {
            class: "refills__eligible-supplies",
            // EligibleSuppliesList { }
            // EligibleSuppliesCommands {
                // on_submit: move |_| {}
            // }
        }
    }
}

// #[component]
// fn EligibleSuppliesList() -> Element {
//     let draft = use_context::<Signal<DraftRefill>>();

//     rsx! {
//         ul {
//             class: "refills__eligible-supplies__list",
//             for (script_id, refills) in draft().items {
//                 EligibleSuppliesListItem { script_id, refills }
//             }
//         }
//     }
// }

// #[component]
// fn EligibleSuppliesListItem(script_id: ScriptId, refills: Vec<DraftRefillItem>) -> Element {
//     let logbook = use_context::<Signal<Logbook>>();
//     let script = logbook
//         .read()
//         .script(script_id)
//         .map_or("".into(), |s| s.to_string());

//     rsx! {
//         li {
//             class: "refills__eligible-supplies__list-item",
//             {script}
//             EligibleMedicationsList { refills }
//         }
//     }
// }

// #[component]
// fn EligibleMedicationsList(refills: Vec<DraftRefillItem>) -> Element {
//     rsx! {
//         ul {
//             class: "refills__eligible-medications__list",
//             for refill in refills {
//                 EligibleMedicationsListItem { refill }
//             }
//         }
//     }
// }

// #[component]
// fn EligibleMedicationsListItem(refill: DraftRefillItem) -> Element {
//     let logbook = use_context::<Signal<Logbook>>();
//     let medication = logbook
//         .read()
//         .medication(refill.medication_id())
//         .map_or("".into(), |m| m.to_string());

//     info!("{} {}", refill.medication_id(), refill.remaining());

//     rsx! {
//         li {
//             class: "refills__eligible-medications__list-item",
//             input { r#type: "checkbox" }
//             span { "{medication}" }
//             span { {tid!("remaining-supplies", n: refill.remaining().to_string())} }
//         }
//     }
// }

// #[component]
// fn EligibleSuppliesCommands(on_submit: EventHandler<Supply>) -> Element {
//     let draft = use_context::<Signal<DraftRefill>>();

//     rsx! {
//         button {
//             title: tid!("dispensed-button.hint"),
//             aria_label: tid!("dispensed-button.aria-label"),
//             onclick: move |_| on_submit.call(draft.read().into_supply()),
//             {tid!("dispensed-button.text")}
//         }
//     }
// }

#[component]
fn PreviousSupplies() -> Element {
    rsx! {
        div {
            class: "refills__previous-supplies",
            "Previous Supplies"
        }
    }
}
