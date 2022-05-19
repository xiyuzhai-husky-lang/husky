use std::sync::Arc;

use word::CustomIdentifier;

use crate::*;

pub type EvalResult<'eval> = VMRuntimeResult<EvalValue<'eval>>;

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
    pub fn as_primitive(&self) -> PrimitiveValue {
        match self {
            EvalValue::Primitive(value) => *value,
            EvalValue::Boxed(_) => todo!(),
            EvalValue::GlobalPure(_) => todo!(),
            EvalValue::GlobalRef(_) => todo!(),
            EvalValue::Undefined => todo!(),
        }
    }

    pub fn into_boxed(self) -> VMRuntimeResult<BoxedValue<'eval>> {
        match self {
            EvalValue::Boxed(value) => Ok(value),
            _ => todo!(),
        }
    }

    pub fn into_stack(self) -> VMRuntimeResult<StackValue<'eval, 'eval>> {
        match self {
            EvalValue::Primitive(value) => Ok(StackValue::Primitive(value)),
            EvalValue::Boxed(value) => Ok(StackValue::Boxed(value)),
            EvalValue::GlobalPure(value) => Ok(StackValue::GlobalPure(value)),
            EvalValue::GlobalRef(value) => Ok(StackValue::GlobalRef(value)),
            EvalValue::Undefined => todo!(),
        }
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

    pub fn lazy_field_var(mut self, field_idx: usize, contract: LazyContract) -> EvalValue<'eval> {
        match self {
            EvalValue::Primitive(_) => panic!("primitive doesn't have member variables"),
            EvalValue::Boxed(value) => {
                let mut value: VirtualTy = value.take().unwrap();
                value.take_field_var(field_idx).into_eval()
            }
            EvalValue::GlobalPure(_) => panic!("expect global ref"),
            EvalValue::GlobalRef(value) => unsafe {
                value
                    .downcast_ref::<VirtualTy<'eval>>()
                    .eval_field_var(field_idx)
                    .share_globally()
            },
            EvalValue::Undefined => todo!(),
        }
    }

    // unsafe fn share_globally(&self) -> EvalValue<'eval> {
    //     match self {
    //         EvalValue::Primitive(value) => EvalValue::Primitive(*value),
    //         EvalValue::GlobalRef(value) => EvalValue::GlobalRef(*value),
    //         EvalValue::GlobalPure(value) => EvalValue::GlobalPure(value.clone()),
    //         EvalValue::Undefined => EvalValue::Undefined,
    //         EvalValue::Boxed(_) => todo!(),
    //     }
    // }

    pub fn share(&self) -> EvalValue<'eval> {
        match self {
            EvalValue::Primitive(_) => todo!(),
            EvalValue::Boxed(_) => todo!(),
            EvalValue::GlobalPure(_) => self.clone(),
            EvalValue::GlobalRef(_) => todo!(),
            EvalValue::Undefined => todo!(),
        }
    }

    pub fn any_ref(&self) -> &dyn AnyValueDyn<'eval> {
        match self {
            EvalValue::Primitive(value) => value.any_ref(),
            EvalValue::Boxed(value) => value.any_ref(),
            EvalValue::GlobalPure(value) => &**value,
            EvalValue::GlobalRef(_) => todo!(),
            EvalValue::Undefined => todo!(),
        }
    }
}
