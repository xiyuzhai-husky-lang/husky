use common::*;
use syntax_types::PrimitiveValue;

use crate::*;
use interpret::StackValue;

use super::{
    feature::{Feature, FeatureId, FeatureKind},
    value::{CachedValue, CachedValueStorage},
    *,
};

pub trait Evaluator<'eval> {
    fn feature_kind(&self, feature_id: FeatureId) -> &FeatureKind;
    fn cache(&self, feature_id: FeatureId, value: CachedValueStorage<'eval>) -> CachedValue<'eval>;
    fn eval(&self, feature_id: FeatureId) -> StackValue<'eval> {
        const VOID: () = ();
        match self.feature_kind(feature_id) {
            FeatureKind::Literal(literal) => literal.into(),
            FeatureKind::FunctionCall => todo!(),
            // FeatureKind::PatternCall => todo!(),
            FeatureKind::Binary => todo!(),
            FeatureKind::MembAccess => todo!(),
            FeatureKind::MembCall => todo!(),
        }
    }
}
