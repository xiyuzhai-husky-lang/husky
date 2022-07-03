mod division;
mod tests;

use husky_compile_time::HuskyCompileTime;
use husky_eager_semantics::FuncStmt;
use husky_package_semantics::{Config, Package};
use trivial_iter::TrivialIter;
use vm::{eval_fast, InterpreterQueryGroup, Mode, __EvalResult};
use word::RootIdentifier;

use crate::*;

use std::{
    any::{Any as _, TypeId},
    ops::Index,
    sync::Arc,
};

use division::Division;
use husky_datasets_protocol::{Dataset, DatasetDyn};
use husky_feature_gen::{Feature, FeaturePtr};

#[derive(Debug)]
pub struct Session<'eval> {
    config: Arc<Config>,
    pub(crate) dataset: Dataset<'eval>,
    pub(crate) dev: Division<'eval>,
    pub(crate) trained_features: Mutex<HashMap<EvalKey<'eval>, EvalValueResult<'eval>>>,
    val: Division<'eval>,
    test: Division<'eval>,
    validation_report: ValidationReport<'eval>,
}

#[derive(Debug)]
pub struct ValidationReport<'sess> {
    predictions: Vec<EvalValueResult<'sess>>,
}

impl<'eval> Default for ValidationReport<'eval> {
    fn default() -> Self {
        Self {
            predictions: vec![],
        }
    }
}

impl<'eval> Session<'eval> {
    pub fn new(
        package: &Package,
        db: &dyn FeatureGenQueryGroup,
        vm_config: &VMConfig,
    ) -> __EvalResult<Self> {
        let config = package.config.clone();
        let dataset: Dataset = eval_fast(
            db.upcast(),
            Some(&db.dataset_config_instruction_sheet(package.main_defn.file)),
            None,
            RootIdentifier::DatasetType.into(),
            [].into_iter(),
            [].into_iter(),
            vm_config,
        )?
        .owned()?
        .downcast_move()?;
        Ok(Self {
            dev: Division::new(dataset.dev_loader()),
            val: Division::new(dataset.val_loader()),
            test: Division::new(dataset.test_loader()),
            validation_report: Default::default(),
            config,
            dataset,
            trained_features: Default::default(),
        })
    }

    pub fn dev(&self) -> &Division<'eval> {
        &self.dev
    }
}
