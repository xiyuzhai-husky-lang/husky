use crate::*;
use print_utils::msg_once;
use std::sync::Arc;
use word::CustomIdentifier;

// EvalValue lives on its own, i.e. not depending on stack context
#[derive(Debug, Clone)]
pub enum __EvalValue<'eval> {
    Copyable(CopyableValue),
    Owned(__OwnedValue<'eval, 'eval>),
    EvalPure(Arc<dyn __AnyValueDyn<'eval> + 'eval>),
    EvalRef(__EvalRef<'eval>),
    Undefined,
    Unreturned, // use for control flow
}

#[derive(Debug, Clone, Copy)]
pub struct __EvalRef<'eval>(pub &'eval (dyn __AnyValueDyn<'eval> + 'eval));

impl<'eval1, 'eval2: 'eval1> __EvalRef<'eval2> {
    pub fn short(&self) -> __EvalRef<'eval1> {
        __EvalRef(self.0.__short_dyn())
    }
}

impl<'eval> PartialEq for __EvalRef<'eval> {
    fn eq(&self, other: &Self) -> bool {
        self.0 as *const (dyn __AnyValueDyn<'eval> + 'eval)
            == other.0 as *const (dyn __AnyValueDyn<'eval> + 'eval)
    }
}

impl<'eval> Eq for __EvalRef<'eval> {}

impl<'eval> std::ops::Deref for __EvalRef<'eval> {
    type Target = dyn __AnyValueDyn<'eval> + 'eval;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'eval> std::hash::Hash for __EvalRef<'eval> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.0 as *const (dyn __AnyValueDyn<'eval> + 'eval) as *const u8).hash(state);
    }
}

impl<'eval> PartialEq for __EvalValue<'eval> {
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

impl<'eval> Eq for __EvalValue<'eval> {}

impl<'eval> From<CopyableValue> for __EvalValue<'eval> {
    fn from(value: CopyableValue) -> Self {
        Self::Copyable(value)
    }
}

impl<'eval> Into<TraceTokenData> for __EvalValue<'eval> {
    fn into(self) -> TraceTokenData {
        match self {
            __EvalValue::Copyable(value) => fade!(value),
            __EvalValue::Owned(value) => fade!(value.any_ref().__print_short()),
            __EvalValue::EvalPure(value) => fade!(value.__print_short()),
            __EvalValue::EvalRef(value) => fade!(value.__print_short()),
            __EvalValue::Undefined => fade!("undefined"),
            __EvalValue::Unreturned => panic!(),
        }
    }
}

impl<'eval> __EvalValue<'eval> {
    pub fn primitive(&self) -> CopyableValue {
        match self {
            __EvalValue::Copyable(value) => *value,
            __EvalValue::EvalRef(value) => value.__take_copyable_dyn(),
            _ => {
                p!(self);
                panic!()
            }
        }
    }

    pub fn owned(self) -> __EvalResult<__OwnedValue<'eval, 'eval>> {
        match self {
            __EvalValue::Owned(value) => Ok(value),
            _ => todo!(),
        }
    }

    pub fn into_stack<'stack>(self) -> __EvalResult<__TempValue<'stack, 'eval>> {
        match self {
            __EvalValue::Copyable(value) => Ok(__TempValue::Copyable(value)),
            __EvalValue::Owned(value) => Ok(__TempValue::OwnedEval(value)),
            __EvalValue::EvalPure(value) => Ok(__TempValue::EvalPure(value)),
            __EvalValue::EvalRef(value) => Ok(__TempValue::EvalRef(value)),
            __EvalValue::Undefined => todo!(),
            __EvalValue::Unreturned => panic!(),
        }
    }

    pub fn snapshot(&self) -> StackValueSnapshot<'eval> {
        msg_once!("deprecated");
        match self {
            __EvalValue::Copyable(value) => StackValueSnapshot::Copyable(*value),
            __EvalValue::Owned(value) => StackValueSnapshot::Owned(value.clone()),
            __EvalValue::EvalPure(value) => StackValueSnapshot::EvalPure(value.clone()),
            __EvalValue::EvalRef(value) => StackValueSnapshot::EvalRef(*value),
            __EvalValue::Undefined => todo!(),
            __EvalValue::Unreturned => panic!(),
        }
    }

    pub fn field_access(mut self, field_idx: usize, binding: Binding) -> __EvalValue<'eval> {
        msg_once!("use field binding");
        match self {
            __EvalValue::Copyable(_) => panic!("primitive doesn't have member variables"),
            __EvalValue::Owned(value) => {
                let mut value: VirtualStruct = value.downcast_move().unwrap();
                value.take_field(field_idx).into_eval()
            }
            __EvalValue::EvalPure(_) => panic!("expect global ref"),
            __EvalValue::EvalRef(value) => unsafe {
                value
                    .0
                    .__downcast_ref::<VirtualStruct<'eval>>()
                    .eval_field(field_idx)
                    .share_globally()
            },
            __EvalValue::Undefined => todo!(),
            __EvalValue::Unreturned => panic!(),
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

    pub fn share(&self) -> __EvalValue<'eval> {
        match self {
            __EvalValue::Copyable(_) => todo!(),
            __EvalValue::Owned(_) => todo!(),
            __EvalValue::EvalPure(_) => self.clone(),
            __EvalValue::EvalRef(_) => todo!(),
            __EvalValue::Undefined => todo!(),
            __EvalValue::Unreturned => panic!(),
        }
    }

    pub fn any_ref(&self) -> &(dyn __AnyValueDyn<'eval> + 'eval) {
        match self {
            __EvalValue::Copyable(value) => value.any_ref(),
            __EvalValue::Owned(value) => value.any_ref(),
            __EvalValue::EvalPure(value) => &**value,
            __EvalValue::EvalRef(value) => value.0,
            __EvalValue::Undefined => todo!(),
            __EvalValue::Unreturned => panic!(),
        }
    }

    pub fn eval_ref(&self) -> __EvalRef<'eval> {
        match self {
            __EvalValue::Copyable(value) => panic!(),
            __EvalValue::Owned(value) => panic!(),
            __EvalValue::EvalPure(value) => panic!(),
            __EvalValue::EvalRef(value) => *value,
            __EvalValue::Undefined => todo!(),
            __EvalValue::Unreturned => panic!(),
        }
    }
}
