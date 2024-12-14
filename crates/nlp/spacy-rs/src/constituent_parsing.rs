pub mod constituent;
pub mod error;
pub mod parser;
#[cfg(test)]
mod tests;

use self::error::*;
use crate::*;
use pyo3::{conversion::FromPyObjectBound, prelude::*, types::PyList};

pub enum Constituent {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromPyObject)]
pub struct ConstituentParsingOutput {
    pub tokens: Vec<SpacyToken>,
    // pub constituents: Vec<String>,
    pub parse_string: String,
}
