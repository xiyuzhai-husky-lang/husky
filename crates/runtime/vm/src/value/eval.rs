use std::sync::Arc;

use print_utils::msg_once;
use word::CustomIdentifier;

use crate::*;

pub type EvalResult<'eval> = VMRuntimeResult<EvalValue<'eval>>;

// EvalValue lives on its own, i.e. not depending on stack context
#[derive(Debug, Clone)]
pub enum EvalValue<'eval> {
    Copyable(CopyableValue),
    Owned(OwnedValue<'eval, 'eval>),
    GlobalPure(Arc<dyn AnyValueDyn<'eval> + 'eval>),
    EvalRef(&'eval (dyn AnyValueDyn<'eval> + 'eval)),
    Undefined,
}

impl<'eval> PartialEq for EvalValue<'eval> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Copyable(l0), Self::Copyable(r0)) => l0 == r0,
            (Self::Owned(l0), Self::Owned(r0)) => l0 == r0,
            (Self::GlobalPure(l0), Self::GlobalPure(r0)) => todo!(),
            (Self::EvalRef(l0), Self::EvalRef(r0)) => todo!(),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl<'eval> Eq for EvalValue<'eval> {}

impl<'eval> From<CopyableValue> for EvalValue<'eval> {
    fn from(value: CopyableValue) -> Self {
        Self::Copyable(value)
    }
}

impl<'eval> EvalValue<'eval> {
    pub fn primitive(&self) -> CopyableValue {
        match self {
            EvalValue::Copyable(value) => *value,
            EvalValue::Owned(_) => todo!(),
            EvalValue::GlobalPure(_) => todo!(),
            EvalValue::EvalRef(_) => todo!(),
            EvalValue::Undefined => todo!(),
        }
    }

    pub fn owned(self) -> VMRuntimeResult<OwnedValue<'eval, 'eval>> {
        match self {
            EvalValue::Owned(value) => Ok(value),
            _ => todo!(),
        }
    }

    pub fn into_stack<'stack>(self) -> VMRuntimeResult<TempValue<'stack, 'eval>> {
        match self {
            EvalValue::Copyable(value) => Ok(TempValue::Copyable(value)),
            EvalValue::Owned(value) => Ok(TempValue::EvalOwned(value)),
            EvalValue::GlobalPure(value) => Ok(TempValue::EvalPure(value)),
            EvalValue::EvalRef(value) => Ok(TempValue::EvalRef(value)),
            EvalValue::Undefined => todo!(),
        }
    }

    pub fn snapshot(&self) -> StackValueSnapshot<'eval> {
        match self {
            EvalValue::Copyable(value) => StackValueSnapshot::Copyable(*value),
            EvalValue::Owned(_) => todo!(),
            EvalValue::GlobalPure(_) => todo!(),
            EvalValue::EvalRef(_) => todo!(),
            EvalValue::Undefined => todo!(),
        }
    }

    pub fn lazy_field(mut self, field_idx: usize, field_binding: Binding) -> EvalValue<'eval> {
        msg_once!("use field binding");
        match self {
            EvalValue::Copyable(_) => panic!("primitive doesn't have member variables"),
            EvalValue::Owned(value) => {
                let mut value: VirtualTy = value.take().unwrap();
                value.take_field(field_idx).into_eval()
            }
            EvalValue::GlobalPure(_) => panic!("expect global ref"),
            EvalValue::EvalRef(value) => unsafe {
                value
                    .downcast_ref::<VirtualTy<'eval>>()
                    .eval_field(field_idx)
                    .share_globally()
            },
            EvalValue::Undefined => todo!(),
        }
    }

    // unsafe fn share_globally(&self) -> EvalValue<'eval> {
    //     match self {
    //         EvalValue::Primitive(value) => EvalValue::Primitive(*value),
    //         EvalValue::EvalRef(value) => EvalValue::EvalRef(*value),
    //         EvalValue::GlobalPure(value) => EvalValue::GlobalPure(value.clone()),
    //         EvalValue::Undefined => EvalValue::Undefined,
    //         EvalValue::Boxed(_) => todo!(),
    //     }
    // }

    pub fn share(&self) -> EvalValue<'eval> {
        match self {
            EvalValue::Copyable(_) => todo!(),
            EvalValue::Owned(_) => todo!(),
            EvalValue::GlobalPure(_) => self.clone(),
            EvalValue::EvalRef(_) => todo!(),
            EvalValue::Undefined => todo!(),
        }
    }

    pub fn any_ref(&self) -> &(dyn AnyValueDyn<'eval> + 'eval) {
        match self {
            EvalValue::Copyable(value) => value.any_ref(),
            EvalValue::Owned(value) => value.any_ref(),
            EvalValue::GlobalPure(value) => &**value,
            EvalValue::EvalRef(value) => *value,
            EvalValue::Undefined => todo!(),
        }
    }

    pub fn any_global_ref(&self) -> &'eval (dyn AnyValueDyn<'eval> + 'eval) {
        match self {
            EvalValue::Copyable(value) => panic!(),
            EvalValue::Owned(value) => panic!(),
            EvalValue::GlobalPure(value) => panic!(),
            EvalValue::EvalRef(value) => *value,
            EvalValue::Undefined => todo!(),
        }
    }
}
