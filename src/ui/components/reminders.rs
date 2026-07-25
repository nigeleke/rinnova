use dioxus::prelude::*;

use crate::application::Model;

#[component]
pub fn Reminders() -> Element {
    let mut _model = use_context::<Signal<Model>>();

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/reminders.css")}
        div {
            class: "reminders",
            "Reminders"
        }
    }
}
