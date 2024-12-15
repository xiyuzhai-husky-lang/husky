pub mod span;

use crate::*;
use pyo3::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromPyObject)]
pub struct Token {
    pub text: String,
    pub i: usize,
}
