use dioxus::prelude::*;

#[component]
pub fn Scripts() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/scripts.css")}
        div {
            class: "scripts",
            "Prescriptions"
        }

    }
}
