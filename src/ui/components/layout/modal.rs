use dioxus::prelude::*;

#[component]
pub fn Modal(children: Element, on_close: EventHandler<()>) -> Element {
    rsx! {
        document::Stylesheet { href: asset!("assets/css/modal.css") }
        div {
            class: "modal",
            div {
                class: "modal__content",
                onclick: move |e| e.stop_propagation(),
                {children}
            }
        }
    }
}
