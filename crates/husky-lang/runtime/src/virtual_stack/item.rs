use hir::*;

use crate::*;

#[derive(Debug)]
pub enum VirtualStackItem {
    Undefined,
    Primitive(PrimitiveValue),
    Owned(Box<dyn std::any::Any>),
    Ref(&'static dyn std::any::Any),
    MutRef(&'static mut dyn std::any::Any),
}

impl From<PrimitiveValue> for VirtualStackItem {
    fn from(value: PrimitiveValue) -> Self {
        VirtualStackItem::Primitive(value)
    }
}

impl From<&PrimitiveValue> for VirtualStackItem {
    fn from(value: &PrimitiveValue) -> Self {
        VirtualStackItem::Primitive(*value)
    }
}

impl Default for VirtualStackItem {
    fn default() -> Self {
        todo!()
    }
}

impl VirtualStackItem {
    pub(super) fn as_input(&self, contract: InputContract) -> RuntimeResult<Self> {
        match contract {
            InputContract::Intact => todo!(),
            InputContract::Share => todo!(),
            InputContract::Own => todo!(),
        }
    }

    pub(super) fn as_primitive(&self) -> RuntimeResult<PrimitiveValue> {
        if let Self::Primitive(value) = self {
            Ok(*value)
        } else {
            todo!()
        }
    }

    pub(super) fn as_i32(&self) -> RuntimeResult<i32> {
        if let PrimitiveValue::I32(i) = self.as_primitive()? {
            Ok(i)
        } else {
            todo!()
        }
    }

    pub(super) fn as_f32(&self) -> RuntimeResult<f32> {
        if let PrimitiveValue::F32(f) = self.as_primitive()? {
            Ok(f)
        } else {
            todo!()
        }
    }

    pub(super) fn as_u32(&self) -> RuntimeResult<u32> {
        if let PrimitiveValue::B32(f) = self.as_primitive()? {
            Ok(f)
        } else {
            todo!()
        }
    }

    pub(super) fn as_bool(&self) -> RuntimeResult<bool> {
        if let PrimitiveValue::Bool(b) = self.as_primitive()? {
            Ok(b)
        } else {
            todo!()
        }
    }
}
