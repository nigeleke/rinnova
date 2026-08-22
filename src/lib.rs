#![feature(coverage_attribute)]

mod application;
mod domain;
mod i18n;
mod macros;
mod storage;
mod ui;

#[cfg(test)]
mod test_support;

pub use ui::App;
