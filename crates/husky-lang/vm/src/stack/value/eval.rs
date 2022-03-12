use std::sync::Arc;

use crate::*;

pub type EvalResult<'eval> = VMResult<EvalValue<'eval>>;

#[derive(Debug, Clone)]
pub enum EvalValue<'eval> {
    Primitive(PrimitiveValue),
    Boxed(BoxedValue<'eval>),
    Volatile(Arc<dyn AnyValueDyn>),
    GlobalRef(&'eval dyn AnyValueDyn),
    Undefined,
}

impl<'eval> From<PrimitiveValue> for EvalValue<'eval> {
    fn from(value: PrimitiveValue) -> Self {
        Self::Primitive(value)
    }
}

impl<'eval> EvalValue<'eval> {
    pub fn as_primitive(&self) -> VMResult<PrimitiveValue> {
        match self {
            EvalValue::Primitive(value) => Ok(*value),
            EvalValue::Boxed(_) => todo!(),
            EvalValue::Volatile(_) => todo!(),
            EvalValue::GlobalRef(_) => todo!(),
            EvalValue::Undefined => todo!(),
        }
    }

    pub fn into_boxed(self) -> VMResult<BoxedValue<'eval>> {
        match self {
            EvalValue::Boxed(value) => Ok(value),
            _ => todo!(),
        }
    }

    pub fn into_stack(self) -> VMResult<StackValue<'eval, 'eval>> {
        todo!()
    }

    pub fn snapshot(&self) -> StackValueSnapshot {
        match self {
            EvalValue::Primitive(value) => StackValueSnapshot::Primitive(*value),
            EvalValue::Boxed(_) => todo!(),
            EvalValue::Volatile(_) => todo!(),
            EvalValue::GlobalRef(_) => todo!(),
            EvalValue::Undefined => todo!(),
        }
    }
}
