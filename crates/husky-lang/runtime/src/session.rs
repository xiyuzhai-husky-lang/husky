mod eval;
mod feature;
mod impl_intern;
mod impl_offline_eval;
mod impl_online_eval;
mod impl_train;
mod tests;
mod value;

use common::*;
use semantics::{LazyStmt, Package};

use crate::*;

use dataset::Dataset;

use feature::{Feature, FeatureId, FeatureKind};
use value::CachedValue;

pub struct Session<'sess> {
    dataset: Box<dyn Dataset>,
    features: Vec<Feature<'sess>>,
    feature_ids: HashMap<FeatureKind, FeatureId>,
}

impl<'sess> Session<'sess> {
    pub(crate) fn new(package: &'sess Package) -> Self {
        let mut stack = VirtualStack::new();
        let stmts: &[LazyStmt] = &package.config.dataset.stmts;
        todo!()
    }
}
