mod eval;
mod feature;
mod impl_intern;
mod impl_offline_eval;
mod impl_online_eval;
mod impl_train;
mod tests;
mod value;

use common::*;
use interpret::interpret;
use semantics::{DeclStmt, Package};

use crate::*;

use dataset::Dataset;

use feature::{Feature, FeatureId, FeatureKind};
use value::CachedValueStorage;

pub struct Session<'sess> {
    dataset: Box<dyn Dataset>,
    features: Vec<Feature<'sess>>,
    feature_ids: HashMap<FeatureKind, FeatureId>,
}

impl<'sess> Session<'sess> {
    pub(crate) fn new(package: &'sess Package) -> Self {
        let dataset = interpret(&package.config.dataset.instructions)
            .unwrap()
            .owned()
            .unwrap();
        todo!()
    }
}
