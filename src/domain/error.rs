use thiserror::*;

#[derive(Clone, Debug, Error)]
pub enum LogbookError {
    #[error("error.duplicate-medication")]
    DuplicateMedication(String),
}
