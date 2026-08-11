use dioxus::prelude::*;
use dioxus_i18n::tid;

#[component]
pub fn Version() -> Element {
    let version = env!("CARGO_PKG_VERSION");

    rsx! {
        document::Stylesheet { href: asset!("assets/css/version.css") }
        div {
            class: "version",
            img { src: asset!("/assets/images/rinnova_logo.png")}
            div { {tid!("version", version: version)} }
        }
    }
}
