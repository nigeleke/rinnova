use dioxus::prelude::*;
use dioxus_i18n::tid;

#[component]
pub fn DeleteButton(
    definite_object: String,
    onclick: EventHandler<MouseEvent>,
    #[props(extends = button)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/delete_button.css") }
        button {
            class: "delete-button",
            title: tid!("delete-button.hint", definite_object: &definite_object),
            aria_label: tid!("delete-button.aria-label", definite_object: &definite_object),
            onclick,
            ..attributes,
            {tid!("delete-button.text")}
        }
    }
}
