use common::*;

use crate::*;

use super::{
    eval::Evaluator,
    feature::{Feature, FeatureId, FeatureKind},
    value::{CachedValue, CachedValueStorage},
    *,
};

impl<'sess> Session<'sess> {
    pub(crate) fn train(&self, mut feature: Feature<'sess>) -> Feature<'sess> {
        match feature.kind {
            FeatureKind::Literal(_)
            | FeatureKind::FunctionCall
            | FeatureKind::Binary
            | FeatureKind::MembAccess
            | FeatureKind::MembCall => (),
        }
        feature
    }
}
