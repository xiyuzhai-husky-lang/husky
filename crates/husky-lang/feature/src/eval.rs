use common::*;
use vm::PrimitiveValue;

use crate::{cache::EvalCache, intern::FeatureInterner, *};
use vm::{Conditional, EvalValue, StackValue};

use crate::*;

pub struct Evaluator<'a, 'eval: 'a> {
    features: &'a FeatureInterner,
    cache: &'a EvalCache<'eval>,
}

impl<'a, 'eval: 'a> Evaluator<'a, 'eval> {
    pub(super) fn new(features: &'a FeatureInterner, cache: &'a EvalCache<'eval>) -> Self {
        Self { cache, features }
    }

    pub(super) fn _eval(&self, id: FeatureId) -> EvalValue<'eval, 'eval> {
        match self.features[id] {
            Feature::Input => todo!(),
            Feature::Literal(literal) => Ok(Conditional::Defined(StackValue::Primitive(literal))),
            Feature::Assert { condition } => match self.eval(condition) {
                Ok(_) => Ok(Conditional::Undefined),
                Err(_) => todo!(),
            },
            Feature::Do { first, then } => match self.eval(first)? {
                Conditional::Defined(value) => Ok(Conditional::Defined(value)),
                Conditional::Undefined => self.eval(then),
            },
            Feature::PrimitiveBinaryFunc { func, lopd, ropd } => Ok(Conditional::Defined(
                StackValue::Primitive(func.act_on_primitives(
                    self.eval(lopd)?.defined()?.as_primitive()?,
                    self.eval(ropd)?.defined()?.as_primitive()?,
                )?),
            )),
        }
    }

    pub(super) fn eval(&self, id: FeatureId) -> EvalValue<'eval, 'eval> {
        if id.cached {
            self._eval(id)
        } else {
            self.cache.value(id, || self._eval(id))
        }
    }
}
