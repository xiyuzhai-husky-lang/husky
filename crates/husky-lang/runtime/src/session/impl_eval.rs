use common::*;

use crate::{any::Any, *};

use super::{
    feature::{Feature, FeatureId, FeatureKind},
    value::SessionValue,
    *,
};

impl Session {
    fn eval_safe(&mut self, feature_id: FeatureId, input_id: usize) -> *const dyn Any {
        const VOID: () = ();
        let feature = &mut self.features[feature_id.0];
        match &feature.kind {
            FeatureKind::Literal(literal) => match literal {
                syntax_types::Literal::I32Literal(value) => value,
                syntax_types::Literal::F32Literal(value) => value,
                syntax_types::Literal::Void => &VOID,
            },
            FeatureKind::FunctionCall => todo!(),
            // FeatureKind::PatternCall => todo!(),
            FeatureKind::Binary => todo!(),
            FeatureKind::MembAccess => todo!(),
            FeatureKind::MembCall => todo!(),
        }
    }

    pub(crate) fn eval(&mut self, feature_id: FeatureId, input_id: usize) -> &'static dyn Any {
        unsafe { &*self.eval_safe(feature_id, input_id) }
    }
}
