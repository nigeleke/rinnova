mod theme;

pub use theme::ConfirmationTheme;

//-------------------------------------
use dioxus::prelude::*;

use crate::ui::components::{CancelButton, Modal, OkButton};

#[component]
pub fn Confirmation(
    theme: ConfirmationTheme,
    message: String,
    on_ok: EventHandler<()>,
    on_cancel: EventHandler<()>,
) -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/css/confirmation.css") }
        Modal {
            on_close: on_cancel,
            div {
                class: "confirmation {theme}",

                p { "{message.to_string()}" }

                div {
                    class: "confirmation__commands",
                    CancelButton {
                        onclick: move |_| on_cancel.call(())
                    }
                    OkButton {
                        onclick: move |_| on_ok.call(())
                    }
                }
            }
        }
    }
}
