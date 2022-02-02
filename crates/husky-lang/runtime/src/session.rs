mod cache;
mod division;
mod eval;
mod feature;
mod impl_intern_feature;
mod impl_offline_eval;
mod impl_online_eval;
mod impl_parse_feature;
mod impl_train;
mod impl_update;
mod tests;
mod value;

use cache::EvalCache;
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
use feature::{Feature, FeatureId};

pub struct Session<'sess> {
    config: Arc<semantics::Config>,
    dataset: Box<dyn Dataset>,
    features: FeatureInterner,
    dev: Division<'sess>,
    val: Division<'sess>,
    test: Division<'sess>,
    validation_report: ValidationReport<'sess>,
    main: FeatureId,
}

#[derive(Default)]
pub struct FeatureInterner {
    features: Vec<Feature>,
    ids: HashMap<Feature, FeatureId>,
}

impl Index<FeatureId> for FeatureInterner {
    type Output = Feature;

    fn index(&self, index: FeatureId) -> &Self::Output {
        &self.features[index.raw]
    }
}

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
            features: Default::default(),
            main: todo!(),
        };
        sess.update(package);
        Ok(sess)
    }

    pub fn update(&mut self, package: &Package) {
        self.update_config(&package.config);
        self.update_main(package);
        self.update_validation_report();
    }
}
