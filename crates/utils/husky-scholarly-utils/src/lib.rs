use pyo3::prelude::*;
use pyo3::types::PyDict;

#[derive(Debug, thiserror::Error)]
pub enum ScholarlyError {
    #[error("Python error: {0}")]
    PythonError(#[from] pyo3::PyErr),
    #[error("Conversion error: {0}")]
    ConversionError(String),
}

pub type Result<T> = std::result::Result<T, ScholarlyError>;

pub struct ScholarlyRuntime {
    scholarly: Py<PyAny>,
}

impl ScholarlyRuntime {
    pub fn new() -> Result<Self> {
        Python::with_gil(|py| {
            let scholarly = PyModule::import_bound(py, "scholarly")?
                .getattr("scholarly")?
                .into_py(py);
            Ok(Self { scholarly })
        })
    }

    pub fn search_pubs(&self, query: &str) -> Result<String> {
        Python::with_gil(|py| {
            let search_result = self.scholarly.call_method1(py, "search_pubs", (query,))?;
            let first_pub = search_result.call_method0(py, "__next__")?;
            let bibtex = self.scholarly.call_method1(py, "bibtex", (first_pub,))?;
            bibtex.extract(py)
        })
        .map_err(|e| ScholarlyError::PythonError(e))
    }
}
