use dioxus::prelude::*;
use dioxus_i18n::tid;

#[component]
pub fn EditButton(
    definite_object: String,
    onclick: EventHandler<MouseEvent>,
    #[props(extends = button)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/edit_button.css") }
        button {
            class: "edit-button",
            title: tid!("edit-button.hint", definite_object: &definite_object),
            aria_label: tid!("edit-button.aria-label", definite_object: &definite_object),
            onclick,
            ..attributes,
            {tid!("edit-button.text")}
        }
    }
}
