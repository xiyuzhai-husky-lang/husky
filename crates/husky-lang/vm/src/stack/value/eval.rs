use std::sync::Arc;

use word::CustomIdentifier;

use crate::*;

pub type EvalResult<'eval> = VMResult<EvalValue<'eval>>;

#[derive(Debug, Clone)]
pub enum EvalValue<'eval> {
    Primitive(PrimitiveValue),
    Boxed(BoxedValue<'eval>),
    GlobalPure(Arc<dyn AnyValueDyn<'eval>>),
    GlobalRef(&'eval dyn AnyValueDyn<'eval>),
    Undefined,
}

impl<'eval> PartialEq for EvalValue<'eval> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Primitive(l0), Self::Primitive(r0)) => l0 == r0,
            (Self::Boxed(l0), Self::Boxed(r0)) => l0 == r0,
            (Self::GlobalPure(l0), Self::GlobalPure(r0)) => todo!(),
            (Self::GlobalRef(l0), Self::GlobalRef(r0)) => todo!(),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl<'eval> Eq for EvalValue<'eval> {}

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
            EvalValue::GlobalPure(_) => todo!(),
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

    pub fn snapshot(&self) -> StackValueSnapshot<'eval> {
        match self {
            EvalValue::Primitive(value) => StackValueSnapshot::Primitive(*value),
            EvalValue::Boxed(_) => todo!(),
            EvalValue::GlobalPure(_) => todo!(),
            EvalValue::GlobalRef(_) => todo!(),
            EvalValue::Undefined => todo!(),
        }
    }

    pub fn global_memb_var(&self, ident: CustomIdentifier) -> EvalValue<'eval> {
        match self {
            EvalValue::Primitive(_) => panic!("primitive doesn't have member variables"),
            EvalValue::Boxed(value) => panic!("expect global ref"),
            EvalValue::GlobalPure(_) => panic!("expect global ref"),
            EvalValue::GlobalRef(value) => unsafe {
                value
                    .downcast_ref::<VirtualTy<'eval>>()
                    .memb_var(ident)
                    .share_globally()
            },
            EvalValue::Undefined => todo!(),
        }
    }

    unsafe fn share_globally(&self) -> EvalValue<'eval> {
        match self {
            EvalValue::Primitive(value) => EvalValue::Primitive(*value),
            EvalValue::GlobalRef(value) => EvalValue::GlobalRef(*value),
            EvalValue::GlobalPure(value) => EvalValue::GlobalPure(value.clone()),
            EvalValue::Undefined => EvalValue::Undefined,
            EvalValue::Boxed(_) => todo!(),
        }
    }

    pub fn share(&self) -> EvalValue<'eval> {
        match self {
            EvalValue::Primitive(_) => todo!(),
            EvalValue::Boxed(_) => todo!(),
            EvalValue::GlobalPure(_) => self.clone(),
            EvalValue::GlobalRef(_) => todo!(),
            EvalValue::Undefined => todo!(),
        }
    }
}
