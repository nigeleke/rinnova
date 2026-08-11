use dioxus::{prelude::*, web::WebEventExt};
use dioxus_i18n::tid;
use web_sys::wasm_bindgen::JsCast;

use crate::application::{Model, View};
use crate::ui::components::Version;

#[component]
pub fn TermsPage() -> Element {
    let mut model = use_context::<Signal<Model>>();

    let mut content_ref = use_signal(|| None::<web_sys::HtmlElement>);
    let mut terms_read = use_signal(|| false);
    let mut terms_accepted = use_signal(|| false);

    let mut check_scrolled = move || {
        if let Some(e) = content_ref() {
            let at_bottom = e.scroll_top() + e.client_height() >= e.scroll_height();
            terms_read.set(at_bottom);
        }
    };

    rsx! {
        document::Stylesheet { href: asset!("/assets/css/terms_page.css") }
        div {
            class: "terms-page",
            div {
                class: "terms-page__content",
                onmounted: move |event| content_ref.set(event.as_web_event().dyn_into().ok()),
                onresize: move |_| check_scrolled(),
                onscrollend: move |_| check_scrolled(),
                h1 { {tid!("terms-heading")} }
                p { {tid!("terms-para-01")} }
                p { {tid!("terms-para-02")} }
                p { {tid!("terms-para-03")} }
                p { {tid!("terms-para-04")} }
            }
            div {
                class: "terms-page__confirmation",
                label {
                    class: if !*terms_read.read() { "disabled" },
                    input {
                        r#type: "checkbox",
                        disabled: !*terms_read.read(),
                        checked: *terms_accepted.read(),
                        onchange: move |event| terms_accepted.set(event.checked()),
                    }
                    {tid!("terms-confirmation-checkbox.text")}
                }
                button {
                    disabled: !*terms_accepted.read(),
                    onclick: move |_| {
                        model.write().mark_terms_seen();
                        model.write().set_view(View::Medications);
                    },
                    {tid!("terms-continue-button.text")}
                }
            }
            Version { }
        }
    }
}
