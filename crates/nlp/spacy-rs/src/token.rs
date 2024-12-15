pub mod span;

use crate::*;
use pyo3::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromPyObject)]
pub struct Token {
    pub text: String,
    // pub lemma: String,
    // pub pos: String,
    // pub tag: String,
    // pub dep: String,
    // pub head: usize,
}
