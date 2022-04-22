mod any;
mod boxed;
mod eval;
mod member;
mod primitive;

pub use any::*;
pub use boxed::BoxedValue;
pub use eval::{EvalResult, EvalValue};
pub use member::*;
pub use primitive::PrimitiveValue;
use std::sync::Arc;
use word::CustomIdentifier;

use crate::*;

pub enum StackValue<'stack, 'eval: 'stack> {
    Moved,
    Primitive(PrimitiveValue),
    Boxed(BoxedValue<'eval>),
    GlobalPure(Arc<dyn AnyValueDyn<'eval>>),
    GlobalRef(&'eval dyn AnyValueDyn<'eval>),
    LocalRef(&'stack dyn AnyValueDyn<'eval>),
    MutLocalRef {
        value: &'stack mut dyn AnyValueDyn<'eval>,
        owner: StackIdx,
        gen: MutRefGenerator,
    },
}

pub type MutRefGenerator = ();

impl<'stack, 'eval: 'stack> std::fmt::Debug for StackValue<'stack, 'eval> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackValue::Primitive(arg0) => {
                f.write_str("Primitive ")?;
                arg0.fmt(f)
            }
            StackValue::Boxed(arg0) => f.debug_tuple("Boxed").field(arg0).finish(),
            StackValue::GlobalPure(arg0) => f.debug_tuple("Volatile").field(arg0).finish(),
            StackValue::GlobalRef(arg0) => f.debug_tuple("GlobalRef").field(arg0).finish(),
            StackValue::LocalRef(arg0) => f.debug_tuple("Ref").field(arg0).finish(),
            StackValue::MutLocalRef { value, .. } => f.debug_tuple("MutRef").field(value).finish(),
            StackValue::Moved => f.write_str("Taken"),
        }
    }
}

impl<'stack, 'eval: 'stack> StackValue<'stack, 'eval> {
    pub fn boxed(self) -> VMResult<BoxedValue<'eval>> {
        match self {
            StackValue::Primitive(_) => todo!(),
            StackValue::Boxed(value) => Ok(value),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::LocalRef(_) => todo!(),
            StackValue::MutLocalRef { .. } => todo!(),
            StackValue::GlobalPure(_) => todo!(),
            StackValue::Moved => todo!(),
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
    pub fn from_eval(eval_value: EvalValue<'eval>) -> VMResult<Self> {
        Ok(match eval_value {
            EvalValue::Primitive(value) => Self::Primitive(value),
            EvalValue::Boxed(_) => todo!(),
            EvalValue::GlobalPure(_) => todo!(),
            EvalValue::GlobalRef(value) => Self::GlobalRef(value),
            EvalValue::Undefined => todo!(),
        })
    }

    pub fn into_eval(&mut self) -> EvalValue<'eval> {
        match self {
            StackValue::Primitive(primitive_value) => EvalValue::Primitive(*primitive_value),
            StackValue::Boxed(boxed_value) => match std::mem::replace(self, StackValue::Moved) {
                StackValue::Boxed(boxed_value) => EvalValue::Boxed(boxed_value),
                _ => panic!(),
            },
            StackValue::GlobalPure(_) => todo!(),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::LocalRef(_) | StackValue::MutLocalRef { .. } | StackValue::Moved => {
                panic!()
            }
        }
    }

    pub fn into_member(&mut self) -> MemberValue<'eval> {
        match self {
            StackValue::Primitive(primitive_value) => MemberValue::Primitive(*primitive_value),
            StackValue::Boxed(boxed_value) => match std::mem::replace(self, StackValue::Moved) {
                StackValue::Boxed(boxed_value) => MemberValue::Boxed(boxed_value),
                _ => panic!(),
            },
            StackValue::GlobalPure(_) => todo!(),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::LocalRef(_) | StackValue::MutLocalRef { .. } | StackValue::Moved => {
                panic!()
            }
        }
    }

    pub(crate) unsafe fn bind(&mut self, contract: EagerContract, stack_idx: StackIdx) -> Self {
        match contract {
            EagerContract::Pure => self.pure(),
            EagerContract::Move => self.bind_move(),
            EagerContract::GlobalRef => todo!(),
            EagerContract::TakeMut => todo!(),
            EagerContract::BorrowMut => self.borrow_mut(stack_idx),
            EagerContract::Exec => todo!(),
            EagerContract::LetInit => todo!(),
            EagerContract::VarInit => todo!(),
            EagerContract::Return => self.bind_return(),
        }
    }

    unsafe fn pure(&self) -> Self {
        match self {
            StackValue::Primitive(value) => StackValue::Primitive(*value),
            StackValue::Boxed(value) => {
                let ptr: *const dyn AnyValueDyn = &*value.inner;
                StackValue::LocalRef(&*ptr)
            }
            StackValue::GlobalPure(_) => todo!(),
            StackValue::GlobalRef(value) => StackValue::GlobalRef(*value),
            StackValue::LocalRef(_) => todo!(),
            StackValue::MutLocalRef { .. } => todo!(),
            StackValue::Moved => todo!(),
        }
    }

    pub(crate) fn bind_move(&mut self) -> Self {
        match self {
            StackValue::Moved => todo!(),
            StackValue::Primitive(value) => StackValue::Primitive(*value),
            StackValue::Boxed(_) => std::mem::replace(self, StackValue::Moved),
            StackValue::GlobalPure(_) => todo!(),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::LocalRef(_) => todo!(),
            StackValue::MutLocalRef { value, owner, gen } => todo!(),
        }
    }

    pub(crate) fn bind_return(&mut self) -> Self {
        match self {
            StackValue::Moved => todo!(),
            StackValue::Primitive(value) => Self::Primitive(*value),
            StackValue::Boxed(_) => std::mem::replace(self, StackValue::Moved),
            StackValue::GlobalPure(_) => todo!(),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::LocalRef(_) => todo!(),
            StackValue::MutLocalRef { value, owner, gen } => todo!(),
        }
    }

    unsafe fn borrow_mut(&mut self, self_stack_idx: StackIdx) -> Self {
        Self::MutLocalRef {
            value: &mut *self.any_mut_ptr(),
            owner: self.owner(self_stack_idx).unwrap(),
            gen: (),
        }
    }

    fn owner(&self, self_stack_idx: StackIdx) -> Option<StackIdx> {
        match self {
            StackValue::Primitive(_) | StackValue::Boxed(_) => Some(self_stack_idx),
            StackValue::GlobalRef(_) | StackValue::GlobalPure(_) => None,
            StackValue::LocalRef(_) => todo!(),
            StackValue::MutLocalRef { owner, .. } => Some(*owner),
            StackValue::Moved => todo!(),
        }
    }

    fn any(&self) -> &dyn AnyValueDyn {
        {
            match self {
                StackValue::Primitive(value) => match value {
                    PrimitiveValue::I32(value) => value,
                    PrimitiveValue::F32(value) => value,
                    PrimitiveValue::B32(value) => value,
                    PrimitiveValue::B64(value) => value,
                    PrimitiveValue::Bool(value) => value,
                    PrimitiveValue::Void => todo!(),
                },
                StackValue::Boxed(_) => todo!(),
                StackValue::GlobalPure(_) => todo!(),
                StackValue::GlobalRef(_) => todo!(),
                StackValue::LocalRef(_) => todo!(),
                StackValue::MutLocalRef { .. } => todo!(),
                StackValue::Moved => todo!(),
            }
        }
    }

    fn any_mut_ptr(&mut self) -> *mut dyn AnyValueDyn<'eval> {
        {
            match self {
                StackValue::Primitive(value) => match value {
                    PrimitiveValue::I32(value) => value,
                    PrimitiveValue::F32(value) => value,
                    PrimitiveValue::B32(value) => value,
                    PrimitiveValue::B64(value) => value,
                    PrimitiveValue::Bool(value) => value,
                    PrimitiveValue::Void => todo!(),
                },
                StackValue::Boxed(value) => value.any_mut_ptr(),
                StackValue::GlobalPure(_) => todo!(),
                StackValue::GlobalRef(_) => todo!(),
                StackValue::LocalRef(_) => todo!(),
                StackValue::MutLocalRef { .. } => todo!(),
                StackValue::Moved => todo!(),
            }
        }
    }

    pub fn downcast_ref<T: AnyValue<'eval>>(&self) -> &T {
        match self {
            StackValue::Moved => todo!(),
            StackValue::Primitive(_) => todo!(),
            StackValue::Boxed(_) => todo!(),
            StackValue::GlobalPure(value) => value.downcast_ref(),
            StackValue::GlobalRef(value) => value.downcast_ref(),
            StackValue::LocalRef(value) => value.downcast_ref(),
            StackValue::MutLocalRef { value, owner, gen } => todo!(),
        }
    }

    pub fn downcast_mut<T: AnyValue<'eval>>(&mut self) -> &mut T {
        match self {
            StackValue::Moved => todo!(),
            StackValue::Primitive(_) => todo!(),
            StackValue::Boxed(_)
            | StackValue::GlobalPure(_)
            | StackValue::GlobalRef(_)
            | StackValue::LocalRef(_) => {
                panic!()
            }
            StackValue::MutLocalRef { ref mut value, .. } => value.downcast_mut(),
        }
    }

    pub fn downcast_mut_full<T: AnyValue<'eval>>(&mut self) -> (&mut T, StackIdx, ()) {
        match self {
            StackValue::Moved => todo!(),
            StackValue::Primitive(_) => todo!(),
            StackValue::Boxed(_)
            | StackValue::GlobalPure(_)
            | StackValue::GlobalRef(_)
            | StackValue::LocalRef(_) => {
                panic!()
            }
            StackValue::MutLocalRef {
                ref mut value,
                owner,
                gen,
            } => (value.downcast_mut(), *owner, *gen),
        }
    }

    pub fn as_primitive(&self) -> VMResult<PrimitiveValue> {
        match self {
            StackValue::Primitive(value) => Ok(*value),
            StackValue::Boxed(_) => todo!(),
            StackValue::GlobalPure(_) => todo!(),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::LocalRef(_) => todo!(),
            StackValue::MutLocalRef { value, .. } => Ok(value.as_primitive()),
            StackValue::Moved => todo!(),
        }
    }

    pub fn clone_into_stack(&self) -> StackValue<'stack, 'eval> {
        todo!()
    }

    pub(crate) fn snapshot(&mut self) -> StackValueSnapshot<'eval> {
        match self {
            StackValue::Primitive(value) => StackValueSnapshot::Primitive(*value),
            StackValue::Boxed(value) => {
                todo!()
            }
            StackValue::GlobalPure(_) => todo!(),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::LocalRef(_) => todo!(),
            StackValue::MutLocalRef { value, owner, gen } => StackValueSnapshot::MutRef {
                value: value.snapshot(),
                owner: *owner,
                gen: (),
            },
            StackValue::Moved => todo!(),
        }
    }

    pub fn static_type_id(&self) -> StaticTypeId {
        self.any().static_type_id()
    }

    pub fn field_var(self, field_idx: usize, contract: EagerContract) -> StackValue<'stack, 'eval> {
        match self {
            StackValue::Moved => todo!(),
            StackValue::Primitive(_) => todo!(),
            StackValue::Boxed(boxed_value) => {
                let mut value: VirtualTy = boxed_value.take().unwrap();
                value.take_field_var(field_idx)
            }
            StackValue::GlobalPure(_) => todo!(),
            StackValue::GlobalRef(value) => {
                let value: &VirtualTy = value.downcast_ref();
                value.eager_field_var(field_idx, contract)
            }
            StackValue::LocalRef(value) => {
                let value: &VirtualTy = value.downcast_ref();
                value.eager_field_var(field_idx, contract)
            }
            StackValue::MutLocalRef { value, owner, gen } => {
                let virtual_value: &mut VirtualTy = value.downcast_mut();
                virtual_value.field_var_mut(field_idx, contract, owner)
            }
        }
    }
}
