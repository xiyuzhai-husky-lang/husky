use std::{any::TypeId, sync::Arc};

use common::p;

use crate::*;

pub enum StackValue<'stack, 'eval: 'stack> {
    Taken,
    Primitive(PrimitiveValue),
    Boxed(BoxedValue<'eval>),
    Volatile(Arc<dyn AnyValueDyn>),
    GlobalRef(&'eval dyn AnyValueDyn),
    Ref(&'stack dyn AnyValueDyn),
    MutRef {
        value: &'stack mut dyn AnyValueDyn,
        owner: StackIdx,
        gen: Arc<dyn (Fn(&mut StackValue) -> *mut dyn AnyValueDyn) + Send + Sync + 'static>,
    },
}

impl<'stack, 'eval: 'stack> std::fmt::Debug for StackValue<'stack, 'eval> {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackValue::Primitive(arg0) => arg0.fmt(f),
            StackValue::Boxed(arg0) => f.debug_tuple("Boxed").field(arg0).finish(),
            StackValue::Volatile(arg0) => f.debug_tuple("Volatile").field(arg0).finish(),
            StackValue::GlobalRef(arg0) => f.debug_tuple("GlobalRef").field(arg0).finish(),
            StackValue::Ref(arg0) => f.debug_tuple("Ref").field(arg0).finish(),
            StackValue::MutRef { value, .. } => f.debug_tuple("MutRef").field(value).finish(),
            StackValue::Taken => todo!(),
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
            StackValue::Volatile(_) => todo!(),
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
        match eval_value {
            EvalValue::Primitive(_) => todo!(),
            EvalValue::Boxed(_) => todo!(),
            EvalValue::Volatile(_) => todo!(),
            EvalValue::GlobalRef(_) => todo!(),
            EvalValue::Undefined => todo!(),
        }
    }

    pub fn into_eval(self) -> EvalValue<'eval> {
        match self {
            StackValue::Primitive(primitive_value) => EvalValue::Primitive(primitive_value),
            StackValue::Boxed(boxed_value) => EvalValue::Boxed(boxed_value),
            StackValue::Volatile(_) => todo!(),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::Ref(_) => todo!(),
            StackValue::MutRef { .. } => todo!(),
            StackValue::Taken => todo!(),
        }
    }

    pub(crate) unsafe fn bind(&mut self, contract: Contract, stack_idx: StackIdx) -> Self {
        match contract {
            Contract::Pure => self.pure(),
            Contract::Take => self.take(),
            Contract::Share => todo!(),
            Contract::TakeMut => todo!(),
            Contract::BorrowMut => self.borrow_mut(stack_idx),
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
            StackValue::Volatile(_) => todo!(),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::Ref(_) => todo!(),
            StackValue::MutRef { .. } => todo!(),
            StackValue::Taken => todo!(),
        }
    }

    fn take(&mut self) -> Self {
        match std::mem::replace(self, StackValue::Taken) {
            StackValue::Primitive(value) => StackValue::Primitive(value),
            StackValue::Boxed(_) => todo!(),
            StackValue::Volatile(_) => todo!(),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::Ref(_) => todo!(),
            StackValue::MutRef { value, owner, gen } => todo!(),
            StackValue::Taken => todo!(),
        }
    }

    unsafe fn borrow_mut(&mut self, self_stack_idx: StackIdx) -> Self {
        Self::MutRef {
            value: &mut *self.any_mut_ptr(),
            owner: self.owner(self_stack_idx).unwrap(),
            gen: Arc::new(|value| value.any_mut_ptr()),
        }
    }

    fn owner(&self, self_stack_idx: StackIdx) -> Option<StackIdx> {
        match self {
            StackValue::Primitive(_) | StackValue::Boxed(_) => Some(self_stack_idx),
            StackValue::GlobalRef(_) | StackValue::Volatile(_) => None,
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
                StackValue::Volatile(_) => todo!(),
                StackValue::GlobalRef(_) => todo!(),
                StackValue::Ref(_) => todo!(),
                StackValue::MutRef { .. } => todo!(),
                StackValue::Taken => todo!(),
            }
        }
    }

    fn any_mut_ptr(&mut self) -> *mut dyn AnyValueDyn {
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
                StackValue::Volatile(_) => todo!(),
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
            StackValue::Volatile(_) => todo!(),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::Ref(_) => todo!(),
            StackValue::MutRef { value, .. } => Ok(value.as_primitive()),
            StackValue::Taken => todo!(),
        }
    }

    pub fn clone_any(&self) -> Option<Box<dyn AnyValueDyn>> {
        todo!()
    }

    pub(crate) fn snapshot(&mut self) -> StackValueSnapshot {
        match self {
            StackValue::Primitive(value) => StackValueSnapshot::Primitive(*value),
            StackValue::Boxed(value) => {
                p!(value);
                todo!()
            }
            StackValue::Volatile(_) => todo!(),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::Ref(_) => todo!(),
            StackValue::MutRef { value, owner, gen } => StackValueSnapshot::MutRef {
                value: value.snapshot(),
                owner: *owner,
                gen: gen.clone(),
            },
            StackValue::Taken => todo!(),
        }
    }

    pub fn static_type_id(&self) -> TypeId {
        self.any().static_type_id()
    }
}
