mod division;
mod tests;

use common::*;
use semantics_eager::DeclStmt;
use semantics_package::{Config, Package};
use trivial_iter::TrivialIter;
use vm::{eval_fast, EvalResult, Mode, VMResult};

use crate::*;

use std::{
    any::{Any as _, TypeId},
    ops::Index,
    sync::Arc,
};

use dataset::{Dataset, DatasetDyn};

use division::Division;
use semantics_feature::{Feature, FeaturePtr, FeatureSheet};

#[derive(Debug)]
pub struct Session<'sess> {
    config: Arc<Config>,
    pub(crate) dataset: Dataset,
    pub(crate) dev: Division<'sess>,
    val: Division<'sess>,
    test: Division<'sess>,
    validation_report: ValidationReport<'sess>,
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
    pub(crate) fn new(package: &Package) -> VMResult<Self> {
        let config = package.config.clone();
        let dataset: Dataset = eval_fast(
            TrivialIter::default(),
            &config.dataset.instruction_sheet,
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
