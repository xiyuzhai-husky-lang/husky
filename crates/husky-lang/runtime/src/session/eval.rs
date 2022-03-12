use common::*;
use syntax_types::PrimitiveValue;

use crate::*;
use vm::{Conditional, EvalValue, VMValue};

use super::{
    feature::{Feature, FeatureId},
    *,
};

pub struct Evaluator<'a, 'eval: 'a> {
    features: &'a FeatureInterner,
    cache: &'a EvalCache<'eval>,
}

impl<'a, 'eval: 'a> Evaluator<'a, 'eval> {
    pub(super) fn new(features: &'a FeatureInterner, cache: &'a EvalCache<'eval>) -> Self {
        Self { cache, features }
    }

    pub(super) fn eval(&self, id: FeatureId) -> EvalValue<'eval, 'eval> {
        const VOID: () = ();
        match self.features[id] {
            Feature::Input => todo!(),
            Feature::Literal(literal) => Ok(Conditional::Defined(VMValue::Primitive(literal))),
            Feature::Cached(feature) => self.cache.value(id, || self.eval(feature)),
            Feature::Assert { condition } => match self.eval(condition) {
                Ok(_) => Ok(EvalValue::Undefined),
                Err(_) => todo!(),
            },
            Feature::Do { first, then } => match self.eval(first)? {
                Conditional::Defined(value) => Ok(Conditional::Defined(value)),
                EvalValue::Undefined => self.eval(then),
            },
            Feature::PrimitiveBinaryFunc { func, lopd, ropd } => Ok(Conditional::Defined(
                VMValue::Primitive(func.act_on_primitives(
                    self.eval(lopd)?.defined()?.as_primitive()?,
                    self.eval(ropd)?.defined()?.as_primitive()?,
                )?),
            )),
        }
    }
}
