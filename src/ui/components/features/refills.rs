use dioxus::prelude::*;

#[component]
pub fn Refills() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/refills.css")}
        div {
            class: "refills",
            "Refills"
        }
    }
}
