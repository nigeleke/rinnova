use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Setup {
    #[default]
    Welcome,
    Terms,
    Complete,
}
