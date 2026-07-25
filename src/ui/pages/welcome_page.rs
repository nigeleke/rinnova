use dioxus::prelude::*;
use dioxus_i18n::tid;

use crate::application::Model;

#[component]
pub fn WelcomePage() -> Element {
    let mut model = use_context::<Signal<Model>>();

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/welcome_page.css") }
        div {
            class: "welcome-page",
            div {
                class: "welcome-page__content",
                h1 { {tid!("welcome-heading")} }
                p { {tid!("welcome-para-01")} }
                p { {tid!("welcome-para-02")} }
            }
            button {
                onclick: move |_| model.write().mark_welcome_seen(),
                {tid!("welcome-continue-button.text")}
            }
        }
    }
}
