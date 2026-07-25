use dioxus::prelude::*;

#[component]
pub fn Prescriptions() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/prescriptions.css")}
        div {
            class: "prescriptions",
            "Prescriptions"
        }

    }
}
