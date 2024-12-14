pub mod constituent;
pub mod error;
pub mod parser;
#[cfg(test)]
mod tests;

use self::error::*;
use crate::*;
use pyo3::{conversion::FromPyObjectBound, prelude::*, types::PyList};

pub enum Constituent {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstituentParsingOutput {
    pub tokens: Vec<SpacyToken>,
    // pub constituents: Vec<String>,
    pub parse_string: String,
}

impl<'a, 'py> ConstituentParsingOutput {
    fn from_py_object_bound(ob: Borrowed<'a, 'py, PyAny>) -> PyResult<Self> {
        let tokens = ob.getattr("tokens")?.extract()?;
        // let constituents = ob.getattr("constituents")?;
        let parse_string = ob.getattr("parse_string")?;
        Ok(Self {
            tokens,
            // constituents: constituents.extract()?,
            parse_string: parse_string.extract()?,
        })
    }
}
