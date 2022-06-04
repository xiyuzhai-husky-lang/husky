mod division;
mod tests;

use compile_time_db::{HuskyCompileTime, InstructionGenQueryGroup};
use pack_semantics::{Config, Pack};
use semantics_eager::FuncStmt;
use trivial_iter::TrivialIter;
use vm::{eval_fast, EvalResult, Mode, VMRuntimeResult};

use crate::*;

use std::{
    any::{Any as _, TypeId},
    ops::Index,
    sync::Arc,
};

use datasets::{Dataset, DatasetDyn};

use division::Division;
use feature::{EvalSheet, Feature, FeaturePtr};

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
    pub fn new(
        package: &Pack,
        compile_time: &HuskyCompileTime,
        verbose: bool,
    ) -> VMRuntimeResult<Self> {
        let config = package.config.clone();
        let dataset: Dataset = eval_fast(
            compile_time,
            Some(&compile_time.dataset_config_instruction_sheet(package.main_defn.file)),
            None,
            [].into_iter(),
            [].into_iter(),
            verbose,
        )?
        .owned()?
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
