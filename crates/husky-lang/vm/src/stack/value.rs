mod any;
mod boxed;
mod eval;
mod primitive;

pub use any::*;
pub use boxed::BoxedValue;
pub use eval::{EvalResult, EvalValue};
pub use primitive::PrimitiveValue;
use std::{any::TypeId, sync::Arc};
use word::CustomIdentifier;

use common::p;

use crate::*;

pub enum StackValue<'stack, 'eval: 'stack> {
    Taken,
    Primitive(PrimitiveValue),
    Boxed(BoxedValue<'eval>),
    GlobalPure(Arc<dyn AnyValueDyn<'eval>>),
    GlobalRef(&'eval dyn AnyValueDyn<'eval>),
    Ref(&'stack dyn AnyValueDyn<'eval>),
    MutRef {
        value: &'stack mut dyn AnyValueDyn<'eval>,
        owner: StackIdx,
        gen: MutRefGenerator,
    },
}

pub type MutRefGenerator = ();

impl<'stack, 'eval: 'stack> std::fmt::Debug for StackValue<'stack, 'eval> {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackValue::Primitive(arg0) => {
                f.write_str("Primitive ")?;
                arg0.fmt(f)
            }
            StackValue::Boxed(arg0) => f.debug_tuple("Boxed").field(arg0).finish(),
            StackValue::GlobalPure(arg0) => f.debug_tuple("Volatile").field(arg0).finish(),
            StackValue::GlobalRef(arg0) => f.debug_tuple("GlobalRef").field(arg0).finish(),
            StackValue::Ref(arg0) => f.debug_tuple("Ref").field(arg0).finish(),
            StackValue::MutRef { value, .. } => f.debug_tuple("MutRef").field(value).finish(),
            StackValue::Taken => f.write_str("Taken"),
        }
    }
}

impl<'stack, 'eval: 'stack> StackValue<'stack, 'eval> {
    pub fn boxed(self) -> VMResult<BoxedValue<'eval>> {
        match self {
            StackValue::Primitive(_) => todo!(),
            StackValue::Boxed(value) => Ok(value),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::Ref(_) => todo!(),
            StackValue::MutRef { .. } => todo!(),
            StackValue::GlobalPure(_) => todo!(),
            StackValue::Taken => todo!(),
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
            EvalValue::GlobalRef(_) => todo!(),
            EvalValue::Undefined => todo!(),
        })
    }

    pub fn into_eval(self) -> EvalValue<'eval> {
        match self {
            StackValue::Primitive(primitive_value) => EvalValue::Primitive(primitive_value),
            StackValue::Boxed(boxed_value) => EvalValue::Boxed(boxed_value),
            StackValue::GlobalPure(_) => todo!(),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::Ref(_) | StackValue::MutRef { .. } | StackValue::Taken => panic!(),
        }
    }

    pub(crate) unsafe fn bind(&mut self, contract: EagerContract, stack_idx: StackIdx) -> Self {
        match contract {
            EagerContract::Pure => self.pure(),
            EagerContract::Take => self.take(),
            EagerContract::GlobalRef => todo!(),
            EagerContract::TakeMut => todo!(),
            EagerContract::BorrowMut => self.borrow_mut(stack_idx),
            EagerContract::Exec => todo!(),
        }
        // ,
        //     match self {
        //         StackValue::Primitive(value) => todo!(),
        //         StackValue::Boxed(_) => todo!(),
        //         StackValue::Volatile(_) => todo!(),
        //         StackValue::GlobalRef(_) => todo!(),
        //         StackValue::Ref(_) => todo!(),
        //         StackValue::MutRef{..} => todo!(),
        //     }
    }

    fn pure(&self) -> Self {
        match self {
            StackValue::Primitive(value) => StackValue::Primitive(*value),
            StackValue::Boxed(_) => todo!(),
            StackValue::GlobalPure(_) => todo!(),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::Ref(_) => todo!(),
            StackValue::MutRef { .. } => todo!(),
            StackValue::Taken => todo!(),
        }
    }

    pub(crate) fn take(&mut self) -> Self {
        match self {
            StackValue::Taken => todo!(),
            StackValue::Primitive(value) => StackValue::Primitive(*value),
            StackValue::Boxed(_) => std::mem::replace(self, StackValue::Taken),
            StackValue::GlobalPure(_) => todo!(),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::Ref(_) => todo!(),
            StackValue::MutRef { value, owner, gen } => todo!(),
        }
    }

    unsafe fn borrow_mut(&mut self, self_stack_idx: StackIdx) -> Self {
        Self::MutRef {
            value: &mut *self.any_mut_ptr(),
            owner: self.owner(self_stack_idx).unwrap(),
            gen: (),
        }
    }

    fn owner(&self, self_stack_idx: StackIdx) -> Option<StackIdx> {
        match self {
            StackValue::Primitive(_) | StackValue::Boxed(_) => Some(self_stack_idx),
            StackValue::GlobalRef(_) | StackValue::GlobalPure(_) => None,
            StackValue::Ref(_) => todo!(),
            StackValue::MutRef { owner, .. } => Some(*owner),
            StackValue::Taken => todo!(),
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
                StackValue::Ref(_) => todo!(),
                StackValue::MutRef { .. } => todo!(),
                StackValue::Taken => todo!(),
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
                StackValue::Ref(_) => todo!(),
                StackValue::MutRef { .. } => todo!(),
                StackValue::Taken => todo!(),
            }
        }
    }

    pub fn as_primitive(&self) -> VMResult<PrimitiveValue> {
        match self {
            StackValue::Primitive(value) => Ok(*value),
            StackValue::Boxed(_) => todo!(),
            StackValue::GlobalPure(_) => todo!(),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::Ref(_) => todo!(),
            StackValue::MutRef { value, .. } => Ok(value.as_primitive()),
            StackValue::Taken => todo!(),
        }
    }

    pub fn clone_any(&self) -> Option<Box<dyn AnyValueDyn>> {
        todo!()
    }

    pub(crate) fn snapshot(&mut self) -> StackValueSnapshot<'eval> {
        match self {
            StackValue::Primitive(value) => StackValueSnapshot::Primitive(*value),
            StackValue::Boxed(value) => {
                p!(value);
                todo!()
            }
            StackValue::GlobalPure(_) => todo!(),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::Ref(_) => todo!(),
            StackValue::MutRef { value, owner, gen } => StackValueSnapshot::MutRef {
                value: value.snapshot(),
                owner: *owner,
                gen: (),
            },
            StackValue::Taken => todo!(),
        }
    }

    pub fn static_type_id(&self) -> StaticTypeId {
        self.any().static_type_id()
    }

    pub fn memb_var(
        mut self,
        ident: CustomIdentifier,
        contract: EagerContract,
    ) -> StackValue<'stack, 'eval> {
        match self {
            StackValue::Taken => todo!(),
            StackValue::Primitive(_) => todo!(),
            StackValue::Boxed(boxed_value) => {
                let value: VirtualTy = boxed_value.take().unwrap();
                value.take_memb_var(ident)
            }
            StackValue::GlobalPure(_) => todo!(),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::Ref(_) => todo!(),
            StackValue::MutRef { value, owner, gen } => {
                let virtual_value: &mut VirtualTy = value.downcast_mut();
                virtual_value.memb_var_mut(ident, contract, owner)
            }
        }
    }
}
