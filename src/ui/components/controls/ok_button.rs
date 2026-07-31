use dioxus::prelude::*;
use dioxus_i18n::tid;

#[component]
pub fn OkButton(
    onclick: EventHandler<MouseEvent>,
    #[props(extends = button)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        button {
            class: "ok-button",
            aria_label: tid!("ok-button.aria-label"),
            r#type: "submit",
            onclick,
            ..attributes,
            {tid!("ok-button.text")}
        }
    }
}
