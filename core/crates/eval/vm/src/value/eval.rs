use crate::*;
use print_utils::msg_once;
use std::sync::Arc;
use word::CustomIdentifier;

// EvalValue lives on its own, i.e. not depending on stack context
#[derive(Debug, Clone)]
pub enum EvalValue<'eval> {
    Copyable(CopyableValue),
    Owned(OwnedValue<'eval, 'eval>),
    EvalPure(Arc<dyn __AnyValueDyn<'eval> + 'eval>),
    EvalRef(EvalRef<'eval>),
    Undefined,
}

#[derive(Debug, Clone, Copy)]
pub struct EvalRef<'eval>(pub &'eval (dyn __AnyValueDyn<'eval> + 'eval));

impl<'eval1, 'eval2: 'eval1> EvalRef<'eval2> {
    pub fn short(&self) -> EvalRef<'eval1> {
        EvalRef(self.0.short_dyn())
    }
}

impl<'eval> PartialEq for EvalRef<'eval> {
    fn eq(&self, other: &Self) -> bool {
        self.0 as *const (dyn __AnyValueDyn<'eval> + 'eval)
            == other.0 as *const (dyn __AnyValueDyn<'eval> + 'eval)
    }
}

impl<'eval> Eq for EvalRef<'eval> {}

impl<'eval> std::ops::Deref for EvalRef<'eval> {
    type Target = dyn __AnyValueDyn<'eval> + 'eval;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'eval> std::hash::Hash for EvalRef<'eval> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.0 as *const (dyn __AnyValueDyn<'eval> + 'eval) as *const u8).hash(state);
    }
}

impl<'eval> PartialEq for EvalValue<'eval> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Copyable(l0), Self::Copyable(r0)) => l0 == r0,
            (Self::Owned(l0), Self::Owned(r0)) => l0 == r0,
            (Self::EvalPure(l0), Self::EvalPure(r0)) => todo!(),
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

impl<'eval> Into<TraceTokenData> for EvalValue<'eval> {
    fn into(self) -> TraceTokenData {
        match self {
            EvalValue::Copyable(value) => fade!(value),
            EvalValue::Owned(value) => fade!(value.any_ref().print_short()),
            EvalValue::EvalPure(value) => fade!(value.print_short()),
            EvalValue::EvalRef(value) => fade!(value.print_short()),
            EvalValue::Undefined => fade!("undefined"),
        }
    }
}

impl<'eval> EvalValue<'eval> {
    pub fn primitive(&self) -> CopyableValue {
        match self {
            EvalValue::Copyable(value) => *value,
            EvalValue::EvalRef(value) => value.take_copyable_dyn(),
            _ => panic!(),
        }
    }

    pub fn owned(self) -> __EvalResult<OwnedValue<'eval, 'eval>> {
        match self {
            EvalValue::Owned(value) => Ok(value),
            _ => todo!(),
        }
    }

    pub fn into_stack<'stack>(self) -> __EvalResult<__TempValue<'stack, 'eval>> {
        match self {
            EvalValue::Copyable(value) => Ok(__TempValue::Copyable(value)),
            EvalValue::Owned(value) => Ok(__TempValue::OwnedEval(value)),
            EvalValue::EvalPure(value) => Ok(__TempValue::EvalPure(value)),
            EvalValue::EvalRef(value) => Ok(__TempValue::EvalRef(value)),
            EvalValue::Undefined => todo!(),
        }
    }

    pub fn snapshot(&self) -> StackValueSnapshot<'eval> {
        msg_once!("deprecated");
        match self {
            EvalValue::Copyable(value) => StackValueSnapshot::Copyable(*value),
            EvalValue::Owned(value) => StackValueSnapshot::Owned(value.clone()),
            EvalValue::EvalPure(value) => StackValueSnapshot::EvalPure(value.clone()),
            EvalValue::EvalRef(value) => StackValueSnapshot::EvalRef(*value),
            EvalValue::Undefined => todo!(),
        }
    }

    pub fn field_access(mut self, field_idx: usize, binding: Binding) -> EvalValue<'eval> {
        msg_once!("use field binding");
        match self {
            EvalValue::Copyable(_) => panic!("primitive doesn't have member variables"),
            EvalValue::Owned(value) => {
                let mut value: VirtualStruct = value.take().unwrap();
                value.take_field(field_idx).into_eval()
            }
            EvalValue::EvalPure(_) => panic!("expect global ref"),
            EvalValue::EvalRef(value) => unsafe {
                value
                    .0
                    .downcast_ref::<VirtualStruct<'eval>>()
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
            EvalValue::EvalPure(_) => self.clone(),
            EvalValue::EvalRef(_) => todo!(),
            EvalValue::Undefined => todo!(),
        }
    }

    pub fn any_ref(&self) -> &(dyn __AnyValueDyn<'eval> + 'eval) {
        match self {
            EvalValue::Copyable(value) => value.any_ref(),
            EvalValue::Owned(value) => value.any_ref(),
            EvalValue::EvalPure(value) => &**value,
            EvalValue::EvalRef(value) => value.0,
            EvalValue::Undefined => todo!(),
        }
    }

    pub fn eval_ref(&self) -> EvalRef<'eval> {
        match self {
            EvalValue::Copyable(value) => panic!(),
            EvalValue::Owned(value) => panic!(),
            EvalValue::EvalPure(value) => panic!(),
            EvalValue::EvalRef(value) => *value,
            EvalValue::Undefined => todo!(),
        }
    }
}
