use super::*;
use alien_seed::{attach::attached_seed, AlienSeed};
use attach::Attach;
use disk_cache::{error::DiskCacheError, DiskCache};
use eterned::db::EternerDb;
use husky_pyo3_utils::py_module::py_module_from_path;
use py_modules::get_constituent_parsing_module;
use std::{borrow::Borrow, sync::Arc, sync::OnceLock};

pub struct SpacyConstituentParser<'db> {
    db: &'db EternerDb,
    cache: DiskCache<&'db EternerDb, AlienSeed, String, ConstituentParsingOutput>,
}

impl From<&PyErr> for SpacyConstituentParserError {
    fn from(err: &PyErr) -> Self {
        Self::PyErr2(err.to_string())
    }
}

impl<'db> SpacyConstituentParser<'db> {
    pub fn new(
        db: &'db EternerDb,
        tokio_runtime: Arc<tokio::runtime::Runtime>,
        disk_cache_path: PathBuf,
    ) -> SpacyConstituentParsingResult<Self> {
        Ok(Self {
            db,
            cache: DiskCache::new(db, tokio_runtime.clone(), disk_cache_path)?,
        })
    }
}

impl<'db> SpacyConstituentParser<'db> {
    pub fn parse(
        &self,
        sentence: String,
    ) -> SpacyConstituentParsingResult<ConstituentParsingOutput> {
        self.cache
            .get_or_call(attached_seed(), sentence, |sentence| {
                Python::with_gil(|py| {
                    let py_module: &Bound<PyModule> = get_constituent_parsing_module(py);
                    let output = py_module.call_method1("parse", (sentence,))?;
                    self.db.attach(|| output.extract().map_err(Into::into))
                })
            })
    }
}
