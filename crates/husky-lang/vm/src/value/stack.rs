use std::sync::Arc;

use crate::*;

#[derive(Debug)]
pub enum StackValue<'stack, 'eval: 'stack> {
    Primitive(PrimitiveValue),
    Boxed(BoxedValue<'eval>),
    Volatile(Arc<dyn AnyValueDyn>),
    GlobalRef(&'eval dyn AnyValueDyn),
    Ref(&'stack dyn AnyValueDyn),
    MutRef(&'stack mut dyn AnyValueDyn),
}

impl<'stack, 'eval: 'stack> StackValue<'stack, 'eval> {
    pub fn boxed(self) -> VMResult<BoxedValue<'eval>> {
        match self {
            StackValue::Primitive(_) => todo!(),
            StackValue::Boxed(value) => Ok(value),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::Ref(_) => todo!(),
            StackValue::MutRef(_) => todo!(),
            StackValue::Volatile(_) => todo!(),
        }
    }
}

impl<'stack, 'eval: 'stack> From<PrimitiveValue> for StackValue<'stack, 'eval> {
    fn from(value: PrimitiveValue) -> Self {
        StackValue::Primitive(value)
    }
}

impl<'stack, 'eval: 'stack> From<&PrimitiveValue> for StackValue<'stack, 'eval> {
    fn from(value: &PrimitiveValue) -> Self {
        StackValue::Primitive(*value)
    }
}

impl<'stack, 'eval: 'stack> StackValue<'stack, 'eval> {
    pub(crate) fn as_input(&self, contract: InputContract) -> VMResult<Self> {
        match contract {
            InputContract::Intact => todo!(),
            InputContract::Share => todo!(),
            InputContract::Own => todo!(),
        }
    }

    pub fn as_primitive(&self) -> VMResult<PrimitiveValue> {
        if let Self::Primitive(value) = self {
            Ok(*value)
        } else {
            todo!()
        }
    }

    pub fn clone_any(&self) -> Option<Box<dyn AnyValueDyn>> {
        todo!()
    }
}
