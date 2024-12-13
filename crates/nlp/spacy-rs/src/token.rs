use crate::*;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpacyToken {
    #[pyo3(get)]
    pub text: String,
    #[pyo3(get)]
    pub lemma: String,
    #[pyo3(get)]
    pub pos: String,
    #[pyo3(get)]
    pub tag: String,
    #[pyo3(get)]
    pub dep: String,
    #[pyo3(get)]
    pub head: usize,
}
