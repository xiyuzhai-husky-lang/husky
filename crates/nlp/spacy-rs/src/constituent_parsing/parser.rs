use super::*;
use attach::Attach;
use disk_cache::{error::LlmCacheError, DiskCache};
use eterned::db::EternerDb;
use husky_pyo3_utils::py_module::py_module_from_path;
use std::{borrow::Borrow, sync::OnceLock};

pub struct SpacyConstituentParser<'db> {
    db: &'db EternerDb,
    python_lib_dir: PathBuf,
    py_module: OnceLock<PyResult<Py<PyModule>>>,
    cache: DiskCache<&'db EternerDb, String, ConstituentParsingOutput>,
}

impl From<&PyErr> for SpacyConstituentParserError {
    fn from(err: &PyErr) -> Self {
        Self::PyErr2(err.to_string())
    }
}

impl<'db> SpacyConstituentParser<'db> {
    pub fn new(
        db: &'db EternerDb,
        python_lib_dir: PathBuf,
        disk_cache_path: PathBuf,
    ) -> SpacyConstituentParsingResult<Self> {
        Ok(Self {
            db,
            python_lib_dir,
            py_module: OnceLock::new(),
            cache: DiskCache::new(db, disk_cache_path)?,
        })
    }
}

impl<'db> SpacyConstituentParser<'db> {
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
                self.db.attach(|| output.extract().map_err(Into::into))
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
