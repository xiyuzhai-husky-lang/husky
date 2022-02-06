mod cache;
mod division;
// mod eval;
// mod impl_intern_feature;
// mod impl_offline_eval;
// mod impl_online_eval;
// mod impl_parse_feature;
// mod impl_train;
// mod impl_update;
mod tests;
// mod value;

use common::*;
use semantics::{DeclStmt, Package};
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
    config: Arc<semantics::Config>,
    dataset: Box<dyn Dataset>,
    dev: Division<'sess>,
    val: Division<'sess>,
    test: Division<'sess>,
    validation_report: ValidationReport<'sess>,
    main: FeaturePtr,
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
        let dataset: Box<dyn Dataset> = run(&config.dataset.instructions)?.boxed()?.take()?;
        let mut sess = Self {
            dev: Division::new(dataset.dev_loader()),
            val: Division::new(dataset.val_loader()),
            test: Division::new(dataset.test_loader()),
            validation_report: Default::default(),
            config,
            dataset,
            main: todo!(),
        };
        sess.update(package);
        Ok(sess)
    }

    pub fn update(&mut self, package: &Package) {
        todo!()
        // self.update_config(&package.config);
        // self.update_main(package);
        // self.update_validation_report();
    }
}
