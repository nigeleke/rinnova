use dioxus::prelude::*;
use dioxus_i18n::tid;

#[component]
pub fn CancelButton(
    onclick: EventHandler<MouseEvent>,
    #[props(extends = button)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        button {
            class: "cancel-button",
            aria_label: tid!("cancel-button.aria-label"),
            r#type: "button",
            onclick,
            ..attributes,
            {tid!("cancel-button.text")}
        }
    }
}
