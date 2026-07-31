use dioxus::prelude::*;

use crate::ui::components::{CancelButton, Modal, OkButton};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationTheme {
    Info,
    Warning,
    Destructive,
    Error,
}

impl ConfirmationTheme {
    fn class(self) -> &'static str {
        match self {
            Self::Info => "confirmation-info",
            Self::Warning => "confirmation-warning",
            Self::Destructive => "confirmation-destructive",
            Self::Error => "confirmation-error",
        }
    }
}

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
                class: "confirmation {theme.class()}",

                p { "{message}" }

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
