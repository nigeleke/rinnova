use dioxus::prelude::*;
use dioxus_i18n::prelude::*;

use crate::application::Setup;
use crate::domain::Date;
use crate::domain::LogbookSnapshot;
use crate::i18n;
use crate::storage;
use crate::ui::components::{HomePage, Notification, Notifications, TermsPage, WelcomePage};

#[component]
pub fn App() -> Element {
    let mut model = storage::use_application_model();
    provide_context(model);

    let logbook = storage::use_logbook();
    provide_context(logbook);

    let mut logbook_snapshot = use_signal(|| LogbookSnapshot::default());
    provide_context(ReadSignal::from(logbook_snapshot));

    use_effect(move || {
        let snapshot = LogbookSnapshot::from(&*logbook.read(), Date::today());
        logbook_snapshot.set(snapshot);
    });

    let notifications = use_signal(Vec::<Notification>::default);
    provide_context(notifications);

    let language = i18n::use_preferred_language();
    use_effect(move || {
        if model.read().language().is_none() {
            model.write().set_language(language());
        }
    });

    let mut i18n = use_init_i18n(|| i18n::config(language.read().identifier()));
    use_effect(move || {
        if let Some(language) = model.read().language() {
            i18n.set_language(language.identifier());
        }
    });

    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        document::Stylesheet { href: asset!("/assets/css/main.css") }

        match model.read().setup() {
            Setup::Welcome => rsx! { WelcomePage {} },
            Setup::Terms => rsx! { TermsPage {} },
            Setup::Complete => rsx! { HomePage {} },
        }

        Notifications { }
    }
}
