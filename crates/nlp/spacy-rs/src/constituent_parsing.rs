pub mod constituent;
pub mod error;
pub mod parser;
#[cfg(test)]
mod tests;

use self::error::*;
use crate::*;
use pyo3::prelude::*;

pub enum Constituent {}

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstituentParsingOutput {
    #[pyo3(get)]
    pub tokens: Vec<SpacyToken>,
    #[pyo3(get)]
    pub parse_string: String,
}
