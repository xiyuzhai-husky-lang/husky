mod primitive;

pub use primitive::PrimitiveValue;

use crate::any::Any;

use crate::*;

#[derive(Debug)]
pub enum StackValue<'stack> {
    Primitive(PrimitiveValue),
    Owned(Box<dyn Any>),
    Ref(&'static dyn Any),
    MutRef(&'stack mut dyn Any),
}

impl<'stack> From<PrimitiveValue> for StackValue<'stack> {
    fn from(value: PrimitiveValue) -> Self {
        StackValue::Primitive(value)
    }
}

impl<'stack> From<&PrimitiveValue> for StackValue<'stack> {
    fn from(value: &PrimitiveValue) -> Self {
        StackValue::Primitive(*value)
    }
}

impl<'stack> StackValue<'stack> {
    pub(super) fn as_input(&self, contract: InputContract) -> InterpretResult<Self> {
        match contract {
            InputContract::Intact => todo!(),
            InputContract::Share => todo!(),
            InputContract::Own => todo!(),
        }
    }

    pub(super) fn as_primitive(&self) -> InterpretResult<PrimitiveValue> {
        if let Self::Primitive(value) = self {
            Ok(*value)
        } else {
            todo!()
        }
    }

    pub fn clone_any(&self) -> Option<Box<dyn Any>> {
        todo!()
    }
}

pub enum DurableValue {
    Primitive(PrimitiveValue),
    Owned(Box<dyn Any>),
}

impl DurableValue {
    pub fn owned(self) -> Option<Box<dyn Any>> {
        match self {
            DurableValue::Primitive(_) => None,
            DurableValue::Owned(value) => Some(value),
        }
    }
}
