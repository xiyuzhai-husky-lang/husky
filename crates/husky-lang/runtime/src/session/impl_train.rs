use common::*;

use crate::*;

use super::{
    eval::Evaluator,
    feature::{Feature, FeatureId},
    *,
};

impl<'sess> Session<'sess> {
    pub(crate) fn train(&self, mut feature: Feature) -> Feature {
        match feature {
            Feature::Input
            | Feature::Literal(_)
            | Feature::Assert { .. }
            | Feature::Do { .. }
            | Feature::PrimitiveBinaryFunc { .. }
            | Feature::Cached { .. } => (),
        }
        feature
    }
}
