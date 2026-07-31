use dioxus::prelude::*;
use dioxus_i18n::tid;

#[component]
pub fn AddButton(
    indefinite_object: String,
    onclick: EventHandler<MouseEvent>,
    #[props(extends = button)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/add_button.css") }
        button {
            class: "add-button",
            title: tid!("add-button.hint", indefinite_object: &indefinite_object),
            aria_label: tid!("add-button.aria-label", indefinite_object: &indefinite_object),
            onclick,
            ..attributes,
            {tid!("add-button.text")}
        }
    }
}
