use dioxus::prelude::*;
use dioxus_i18n::tid;

#[component]
pub fn OkButton(
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = button)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        button {
            class: "ok-button",
            aria_label: tid!("ok-button.aria-label"),
            r#type: "submit",
            onclick: move |event| onclick.iter().for_each(|handler| handler.call(event.clone())),
            ..attributes,
            {tid!("ok-button.text")}
        }
    }
}
