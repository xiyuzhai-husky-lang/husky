pub mod constituent;
pub mod error;
pub mod parser;
#[cfg(test)]
mod tests;

use self::{constituent::Constituent, error::*};
use crate::*;
use pyo3::{conversion::FromPyObjectBound, prelude::*, types::PyList};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromPyObject)]
pub struct ConstituentParsingOutput {
    pub tokens: Vec<Token>,
    pub constituent: Constituent,
    pub parse_string: String,
}
