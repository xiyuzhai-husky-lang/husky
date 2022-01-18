use common::*;
use syntax_types::PrimitiveValue;

use crate::*;
use virtual_stack::VirtualStackValue;

use super::{
    feature::{Feature, FeatureId, FeatureKind},
    value::{CachedValue, DurableValue},
    *,
};

pub trait Evaluator<'eval> {
    fn feature_kind(&self, feature_id: FeatureId) -> &FeatureKind;
    fn cache(&self, feature_id: FeatureId, value: CachedValue<'eval>) -> DurableValue<'eval>;
    fn eval(&self, feature_id: FeatureId) -> VirtualStackValue<'eval> {
        const VOID: () = ();

        let mut stack = VirtualStack::new();
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
