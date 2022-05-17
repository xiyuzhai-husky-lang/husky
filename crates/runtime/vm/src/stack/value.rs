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
use print_utils::p;
use std::sync::Arc;
use word::CustomIdentifier;

use crate::*;
use std::fmt::Write;

pub enum StackValue<'stack, 'eval: 'stack> {
    Moved,
    Primitive(PrimitiveValue),
    Boxed(BoxedValue<'eval>),
    GlobalPure(Arc<dyn AnyValueDyn<'eval>>),
    GlobalRef(&'eval dyn AnyValueDyn<'eval>),
    LocalRef {
        value: &'stack dyn AnyValueDyn<'eval>,
        owner: StackIdx,
        gen: MutRefGenerator,
    },
    LocalRefMut {
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
            StackValue::GlobalPure(arg0) => f.debug_tuple("GlobalPure").field(arg0).finish(),
            StackValue::GlobalRef(arg0) => f.debug_tuple("GlobalRef").field(arg0).finish(),
            StackValue::LocalRef { value, .. } => f.debug_tuple("Ref").field(value).finish(),
            StackValue::LocalRefMut { value, .. } => f.debug_tuple("MutRef").field(value).finish(),
            StackValue::Moved => f.write_str("Taken"),
        }
    }
}

impl<'stack, 'eval: 'stack> StackValue<'stack, 'eval> {
    pub fn boxed(self) -> VMRuntimeResult<BoxedValue<'eval>> {
        match self {
            StackValue::Boxed(value) => Ok(value),
            _ => panic!(),
        }
    }

    pub fn print_short(&self) -> String {
        let mut result = String::new();
        match self {
            StackValue::Moved => result.push_str("Moved"),
            StackValue::Primitive(value) => {
                result.push_str("Primitive ");
                result.push_str(&value.any_ref().print_short())
            }
            StackValue::Boxed(value) => {
                result.push_str("Boxed ");
                result.push_str(&value.any_ref().print_short())
            }
            StackValue::GlobalPure(value) => {
                result.push_str("GlobalPure ");
                result.push_str(&value.print_short())
            }
            StackValue::GlobalRef(_) => todo!(),
            StackValue::LocalRef { value, owner, gen } => {
                result.push_str("LocalRef ");
                result.push_str(&value.print_short());
                write!(result, " Owner({:?}) ", owner);
            }
            StackValue::LocalRefMut { value, owner, gen } => {
                result.push_str("LocalRefMut ");
                result.push_str(&value.print_short());
                write!(result, " Owner({:?}) ", owner);
            }
        }
        result
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
    pub fn from_eval(eval_value: EvalValue<'eval>) -> VMRuntimeResult<Self> {
        Ok(match eval_value {
            EvalValue::Primitive(value) => Self::Primitive(value),
            EvalValue::Boxed(_) => todo!(),
            EvalValue::GlobalPure(value) => StackValue::GlobalPure(value),
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
            StackValue::LocalRef { .. } | StackValue::LocalRefMut { .. } | StackValue::Moved => {
                panic!()
            }
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            StackValue::Moved => todo!(),
            StackValue::Primitive(v) => v.to_bool(),
            StackValue::Boxed(_) => todo!(),
            StackValue::GlobalPure(_) => todo!(),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::LocalRef { value, owner, gen } => todo!(),
            StackValue::LocalRefMut { value, owner, gen } => todo!(),
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
            StackValue::LocalRef { .. } | StackValue::LocalRefMut { .. } | StackValue::Moved => {
                panic!()
            }
        }
    }

    pub(crate) unsafe fn bind(&mut self, binding: Binding, stack_idx: StackIdx) -> Self {
        match binding {
            Binding::Ref => self.bind_ref(stack_idx),
            Binding::RefMut => self.bind_ref_mut(stack_idx),
            Binding::Move => self.bind_move(),
            Binding::Copy => self.bind_copy(),
        }
    }

    unsafe fn bind_ref(&self, owner: StackIdx) -> Self {
        match self {
            StackValue::Moved => panic!(),
            StackValue::Primitive(_) => panic!(),
            StackValue::Boxed(value) => {
                let ptr: *const dyn AnyValueDyn = &*value.inner;
                StackValue::LocalRef {
                    value: &*ptr,
                    owner,
                    gen: (),
                }
            }
            StackValue::GlobalPure(value) => {
                let ptr: *const dyn AnyValueDyn = &**value;
                StackValue::LocalRef {
                    value: &*ptr,
                    owner,
                    gen: (),
                }
            }
            StackValue::GlobalRef(value) => StackValue::GlobalRef(*value),
            StackValue::LocalRef { .. } => panic!(),
            StackValue::LocalRefMut { value, owner, gen } => todo!(),
        }
    }

    fn bind_copy(&self) -> Self {
        match self {
            StackValue::Moved => todo!(),
            StackValue::Primitive(value) => StackValue::Primitive(*value),
            StackValue::Boxed(_) => todo!(),
            StackValue::GlobalPure(_) => todo!(),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::LocalRef { value, owner, gen } => todo!(),
            StackValue::LocalRefMut { value, owner, gen } => todo!(),
        }
    }

    unsafe fn bind_ref_mut(&mut self, stack_idx: StackIdx) -> Self {
        match self {
            StackValue::Primitive(value) => {
                let ptr: *mut dyn AnyValueDyn = value.any_mut();
                StackValue::LocalRefMut {
                    value: &mut *ptr,
                    owner: stack_idx,
                    gen: (),
                }
            }
            StackValue::Boxed(value) => {
                let ptr: *mut dyn AnyValueDyn = &mut *value.inner;
                StackValue::LocalRefMut {
                    value: &mut *ptr,
                    owner: stack_idx,
                    gen: (),
                }
            }
            StackValue::Moved
            | StackValue::GlobalPure(_)
            | StackValue::GlobalRef(_)
            | StackValue::LocalRef { .. }
            | StackValue::LocalRefMut { .. } => panic!(),
        }
    }

    unsafe fn pure(&self, stack_idx: StackIdx) -> Self {
        match self {
            StackValue::Primitive(value) => StackValue::Primitive(*value),
            StackValue::Boxed(value) => {
                let ptr: *const dyn AnyValueDyn = &*value.inner;
                StackValue::LocalRef {
                    value: &*ptr,
                    owner: stack_idx,
                    gen: (),
                }
            }
            StackValue::GlobalPure(value) => StackValue::GlobalPure(value.clone()),
            StackValue::GlobalRef(value) => StackValue::GlobalRef(*value),
            StackValue::LocalRef { .. } => todo!(),
            StackValue::LocalRefMut { .. } => todo!(),
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
            StackValue::LocalRef { .. } => todo!(),
            StackValue::LocalRefMut { value, owner, gen } => todo!(),
        }
    }

    pub(crate) fn bind_return(&mut self) -> Self {
        match self {
            StackValue::Moved => todo!(),
            StackValue::Primitive(value) => Self::Primitive(*value),
            StackValue::Boxed(_) => std::mem::replace(self, StackValue::Moved),
            StackValue::GlobalPure(_) => todo!(),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::LocalRef { .. } => todo!(),
            StackValue::LocalRefMut { value, owner, gen } => todo!(),
        }
    }

    unsafe fn borrow_mut(&mut self, self_stack_idx: StackIdx) -> Self {
        Self::LocalRefMut {
            value: &mut *self.any_mut_ptr(),
            owner: self.owner(self_stack_idx).unwrap(),
            gen: (),
        }
    }

    fn owner(&self, self_stack_idx: StackIdx) -> Option<StackIdx> {
        match self {
            StackValue::Primitive(_) | StackValue::Boxed(_) => Some(self_stack_idx),
            StackValue::GlobalRef(_) | StackValue::GlobalPure(_) => None,
            StackValue::LocalRef { .. } => todo!(),
            StackValue::LocalRefMut { owner, .. } => Some(*owner),
            StackValue::Moved => todo!(),
        }
    }

    pub fn any_ref(&self) -> &dyn AnyValueDyn<'eval> {
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
                StackValue::Boxed(value) => value.any_ref(),
                StackValue::GlobalPure(value) => (&**value),
                StackValue::GlobalRef(_) => todo!(),
                StackValue::LocalRef { value, .. } => *value,
                StackValue::LocalRefMut { value, .. } => *value,
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
                StackValue::LocalRefMut { value, .. } => *value,
                StackValue::LocalRef { .. } => panic!("LocalRef cannot be mutated, this is a bug."),
                StackValue::GlobalPure(_) => panic!("GlobalPure cannot be mutated, this is a bug."),
                StackValue::GlobalRef(_) => panic!("GlobalRef cannot be mutated, this is a bug."),
                StackValue::Moved => panic!("Move cannot be mutated, this is a bug."),
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
            StackValue::LocalRef { value, .. } => value.downcast_ref(),
            StackValue::LocalRefMut { value, .. } => value.downcast_ref(),
        }
    }

    pub fn downcast_mut<T: AnyValue<'eval>>(&mut self) -> &mut T {
        match self {
            StackValue::Moved => todo!(),
            StackValue::Primitive(_) => todo!(),
            StackValue::Boxed(_)
            | StackValue::GlobalPure(_)
            | StackValue::GlobalRef(_)
            | StackValue::LocalRef { .. } => {
                panic!()
            }
            StackValue::LocalRefMut { ref mut value, .. } => value.downcast_mut(),
        }
    }

    pub fn downcast_mut_full<T: AnyValue<'eval>>(&mut self) -> (&'stack mut T, StackIdx, ()) {
        match self {
            StackValue::Moved => todo!(),
            StackValue::Primitive(_) => todo!(),
            StackValue::Boxed(_)
            | StackValue::GlobalPure(_)
            | StackValue::GlobalRef(_)
            | StackValue::LocalRef { .. } => {
                panic!()
            }
            StackValue::LocalRefMut { value, owner, gen } => {
                let ptr: *mut T = value.downcast_mut();
                (unsafe { &mut *ptr }, *owner, *gen)
            }
        }
    }

    pub fn as_primitive(&self) -> PrimitiveValue {
        match self {
            StackValue::Primitive(value) => *value,
            StackValue::LocalRefMut { value, .. } => value.as_primitive(),
            _ => {
                p!(self);
                panic!("")
            }
        }
    }

    pub fn clone_into_stack(&self) -> StackValue<'stack, 'eval> {
        match self {
            StackValue::Moved => todo!(),
            StackValue::Primitive(_) => todo!(),
            StackValue::Boxed(_) => todo!(),
            StackValue::GlobalPure(value) => StackValue::Boxed(BoxedValue::clone_from(&**value)),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::LocalRef { value, .. } => Self::Boxed(BoxedValue::clone_from(*value)),
            StackValue::LocalRefMut { value, owner, gen } => todo!(),
        }
    }

    pub(crate) fn snapshot(&mut self) -> StackValueSnapshot<'eval> {
        match self {
            StackValue::Primitive(value) => StackValueSnapshot::Primitive(*value),
            StackValue::Boxed(value) => StackValueSnapshot::Boxed(value.clone()),
            StackValue::GlobalPure(value) => StackValueSnapshot::GlobalPure(value.clone()),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::LocalRef { value, owner, gen } => StackValueSnapshot::Ref {
                value: value.snapshot(),
                owner: *owner,
                gen: *gen,
            },
            StackValue::LocalRefMut { value, owner, gen } => StackValueSnapshot::MutRef {
                value: value.snapshot(),
                owner: *owner,
                gen: *gen,
            },
            StackValue::Moved => todo!(),
        }
    }

    pub fn static_type_id(&self) -> StaticTypeId {
        self.any_ref().static_type_id()
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
            StackValue::LocalRef { value, .. } => {
                let value: &VirtualTy = value.downcast_ref();
                value.eager_field_var(field_idx, contract)
            }
            StackValue::LocalRefMut { value, owner, gen } => {
                let virtual_value: &mut VirtualTy = value.downcast_mut();
                virtual_value.field_var_mut(field_idx, contract, owner)
            }
        }
    }
}
