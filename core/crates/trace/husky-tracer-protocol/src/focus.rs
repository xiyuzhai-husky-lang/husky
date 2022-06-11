use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Focus {
    pub opt_input_id: Option<usize>,
}

impl Default for Focus {
    fn default() -> Self {
        Self { opt_input_id: None }
    }
}
