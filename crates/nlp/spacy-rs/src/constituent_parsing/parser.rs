use super::*;
use disk_cache::{error::LlmCacheError, DiskCache};
use husky_pyo3_utils::py_module_from_path;
use std::sync::OnceLock;

pub struct SpacyConstituentParser {
    python_lib_dir: PathBuf,
    py_module: OnceLock<PyResult<Py<PyModule>>>,
    cache: DiskCache<String, ConstituentParsingOutput>,
}

impl From<&PyErr> for SpacyConstituentParserError {
    fn from(err: &PyErr) -> Self {
        Self::PyErr2(err.to_string())
    }
}

impl SpacyConstituentParser {
    pub fn new(
        python_lib_dir: PathBuf,
        disk_cache_path: PathBuf,
    ) -> SpacyConstituentParsingResult<Self> {
        Ok(Self {
            python_lib_dir,
            py_module: OnceLock::new(),
            cache: DiskCache::new(disk_cache_path)?,
        })
    }
}

impl SpacyConstituentParser {
    pub fn parse(
        &self,
        sentence: String,
    ) -> SpacyConstituentParsingResult<ConstituentParsingOutput> {
        self.cache.get_or_call(sentence, |sentence| {
            Python::with_gil(|py| {
                let py_module = match self.py_module(py) {
                    Ok(py_module) => py_module,
                    Err(err) => todo!(),
                };
                let output = py_module.call_method1("parse", (sentence,))?;
                output.extract().map_err(Into::into)
            })
        })
    }

    fn py_module<'py>(
        &self,
        py: Python<'py>,
    ) -> SpacyConstituentParsingResult<&Bound<'py, PyModule>> {
        Ok(self
            .py_module
            .get_or_init(|| py_module_from_path(&self.python_lib_dir))
            .as_ref()?
            .bind(py))
    }
}
