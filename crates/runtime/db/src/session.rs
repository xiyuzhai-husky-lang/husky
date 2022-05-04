mod division;
mod tests;

use pack_semantics::{Config, Pack};
use semantics_eager::FuncStmt;
use trivial_iter::TrivialIter;
use vm::{eval_fast, EvalResult, Mode, VMResult};

use crate::*;

use std::{
    any::{Any as _, TypeId},
    ops::Index,
    sync::Arc,
};

use datasets::{Dataset, DatasetDyn};

use division::Division;
use feature::{Feature, FeaturePtr, FeatureSheet};

#[derive(Debug)]
pub struct Session<'eval> {
    config: Arc<Config>,
    pub(crate) dataset: Dataset<'eval>,
    pub(crate) dev: Division<'eval>,
    val: Division<'eval>,
    test: Division<'eval>,
    validation_report: ValidationReport<'eval>,
}

#[derive(Debug)]
pub struct ValidationReport<'sess> {
    predictions: Vec<EvalResult<'sess>>,
}

impl<'sess> Default for ValidationReport<'sess> {
    fn default() -> Self {
        Self {
            predictions: vec![],
        }
    }
}

impl<'sess> Session<'sess> {
    pub(crate) fn new(pack: &Pack, compile_time: &HuskyLangCompileTime) -> VMResult<Self> {
        let config = pack.config.clone();
        let dataset: Dataset = eval_fast(
            compile_time,
            TrivialIter::default(),
            Some(&compile_time.dataset_config_instruction_sheet(pack.main_defn.file)),
            None,
        )?
        .into_boxed()?
        .take()?;
        Ok(Self {
            dev: Division::new(dataset.dev_loader()),
            val: Division::new(dataset.val_loader()),
            test: Division::new(dataset.test_loader()),
            validation_report: Default::default(),
            config,
            dataset,
        })
    }
}
