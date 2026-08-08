mod application;
mod domain;
mod i18n;
mod storage;
mod ui;

#[cfg(test)]
mod test_support;

pub use ui::App;

pub mod prelude {
    pub use super::domain::*;
}
