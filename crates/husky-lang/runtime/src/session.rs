mod cache;
mod division;
mod tests;

use common::*;
use semantics::{Config, DeclStmt, Package};
use vm::{run, EvalValue, VMResult};

use crate::*;

use std::{
    any::{Any as _, TypeId},
    ops::Index,
    sync::Arc,
};

use dataset::Dataset;

use division::Division;
use feature::{Feature, FeaturePtr, FeatureSheet};

#[derive(Debug)]
pub struct Session<'sess> {
    config: Arc<Config>,
    pub(crate) dataset: Box<dyn Dataset>,
    pub(crate) dev: Division<'sess>,
    val: Division<'sess>,
    test: Division<'sess>,
    validation_report: ValidationReport<'sess>,
}

#[derive(Debug)]
pub struct ValidationReport<'sess> {
    predictions: Vec<EvalValue<'sess, 'sess>>,
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
        let dataset: Box<dyn Dataset> = run(&config.dataset.instruction_sheet.instructions())?
            .boxed()?
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
