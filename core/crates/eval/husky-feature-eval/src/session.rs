mod division;
mod tests;

pub use division::*;

use crate::*;
use husky_compile_time::HuskyCompileTime;
use husky_datasets_interface::{Dataset, DatasetDyn};
use husky_eager_semantics::FuncStmt;
use husky_feature_gen::{Feature, FeaturePtr};
use husky_package_semantics::{Config, Package};
use husky_word::RootIdentifier;
use std::{
    any::{Any as _, TypeId},
    ops::Index,
    sync::Arc,
};
use trivial_iter::TrivialIter;
use vm::{eval_fast, InterpreterQueryGroup, Mode, __VMResult};

#[derive(Debug)]
pub struct Session<'eval> {
    config: Arc<Config>,
    pub(crate) dataset: Dataset<'eval>,
    pub(crate) dev: Division<'eval>,
    pub(crate) trained_features: Mutex<HashMap<EvalKey, __VMResult<__Register<'eval>>>>,
    val: Division<'eval>,
    test: Division<'eval>,
    validation_report: ValidationReport<'eval>,
}

#[derive(Debug)]
pub struct ValidationReport<'sess> {
    predictions: Vec<__VMResult<__Register<'sess>>>,
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
    ) -> __VMResult<Self> {
        let config = package.config.clone();
        let dataset: Dataset = unsafe {
            eval_fast(
                db.upcast(),
                None,
                Some(&db.dataset_config_instruction_sheet(package.main_defn.file)),
                None,
                RootIdentifier::DatasetType.into(),
                [].into_iter(),
                [].into_iter(),
                0,
                vm_config,
            )?
            .downcast_unbox()
        };
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
