mod view;

pub use view::View;

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiState {
    view: View,
}

impl UiState {
    pub fn view(&self) -> View {
        self.view
    }

    pub fn set_view(&mut self, view: View) {
        self.view = match (self.view, view) {
            (View::HomePage, requested) => requested,
            (current, requested) if current == requested => View::HomePage,
            (_, requested) => requested,
        };
    }
}
