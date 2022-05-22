mod any;
mod copyable;
mod enum_kind;
mod eval;
mod member;
mod owned;
mod xml;

pub use any::*;
pub use copyable::*;
pub use enum_kind::*;
pub use eval::{EvalResult, EvalValue};
pub use member::*;
pub use owned::*;
pub use xml::*;

use crate::*;
use print_utils::{msg_once, p};
use std::fmt::Write;
use std::sync::Arc;
use word::CustomIdentifier;

pub enum StackValue<'stack, 'eval: 'stack> {
    Moved,
    Copyable(CopyableValue),
    Owned(OwnedValue<'eval>),
    GlobalPure(Arc<dyn AnyValueDyn<'eval>>),
    GlobalRef(&'eval dyn AnyValueDyn<'eval>),
    LocalRef(&'stack dyn AnyValueDyn<'eval>),
    RefMut {
        value: &'stack mut dyn AnyValueDyn<'eval>,
        owner: StackIdx,
        gen: MutRefGenerator,
    },
}

pub type MutRefGenerator = ();

impl<'stack, 'eval: 'stack> std::fmt::Debug for StackValue<'stack, 'eval> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackValue::Copyable(arg0) => {
                f.write_str("Primitive ")?;
                arg0.fmt(f)
            }
            StackValue::Owned(arg0) => f.debug_tuple("Boxed").field(arg0).finish(),
            StackValue::GlobalPure(arg0) => f.debug_tuple("GlobalPure").field(arg0).finish(),
            StackValue::GlobalRef(arg0) => f.debug_tuple("GlobalRef").field(arg0).finish(),
            StackValue::LocalRef(value) => f.debug_tuple("Ref").field(value).finish(),
            StackValue::RefMut { value, .. } => f.debug_tuple("MutRef").field(value).finish(),
            StackValue::Moved => f.write_str("Taken"),
        }
    }
}

impl<'stack, 'eval: 'stack> StackValue<'stack, 'eval> {
    pub fn print_short(&self) -> String {
        let mut result = String::new();
        match self {
            StackValue::Moved => result.push_str("Moved"),
            StackValue::Copyable(value) => {
                result.push_str("Primitive ");
                result.push_str(&value.any_ref().print_short())
            }
            StackValue::Owned(value) => {
                result.push_str("Boxed ");
                result.push_str(&value.any_ref().print_short())
            }
            StackValue::GlobalPure(value) => {
                result.push_str("GlobalPure ");
                result.push_str(&value.print_short())
            }
            StackValue::GlobalRef(_) => todo!(),
            StackValue::LocalRef(value) => {
                result.push_str("LocalRef ");
                result.push_str(&value.print_short());
            }
            StackValue::RefMut { value, owner, gen } => {
                result.push_str("LocalRefMut ");
                result.push_str(&value.print_short());
                write!(result, " Owner({:?}) ", owner);
            }
        }
        result
    }

    pub fn to_json_value(self) -> serde_json::value::Value {
        match self {
            StackValue::Moved => todo!(),
            StackValue::Copyable(_) => todo!(),
            StackValue::Owned(_) => todo!(),
            StackValue::GlobalPure(_) => todo!(),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::LocalRef(value) => value.get_json_value_dyn(),
            StackValue::RefMut { value, owner, gen } => todo!(),
        }
    }
}

impl<'stack, 'eval: 'stack> From<CopyableValue> for StackValue<'stack, 'eval> {
    fn from(value: CopyableValue) -> Self {
        StackValue::Copyable(value)
    }
}

impl<'stack, 'eval: 'stack> From<&CopyableValue> for StackValue<'stack, 'eval> {
    fn from(value: &CopyableValue) -> Self {
        StackValue::Copyable(*value)
    }
}

impl<'stack, 'eval: 'stack> StackValue<'stack, 'eval> {
    pub fn from_eval(eval_value: EvalValue<'eval>) -> VMRuntimeResult<Self> {
        Ok(match eval_value {
            EvalValue::Copyable(value) => Self::Copyable(value),
            EvalValue::Owned(_) => todo!(),
            EvalValue::GlobalPure(value) => StackValue::GlobalPure(value),
            EvalValue::GlobalRef(value) => Self::GlobalRef(value),
            EvalValue::Undefined => todo!(),
        })
    }

    pub fn into_eval(self) -> EvalValue<'eval> {
        match self {
            StackValue::Copyable(copyable_value) => EvalValue::Copyable(copyable_value),
            StackValue::Owned(boxed_value) => EvalValue::Owned(boxed_value),
            StackValue::GlobalPure(_) => todo!(),
            StackValue::GlobalRef(value) => EvalValue::GlobalRef(value),
            StackValue::LocalRef { .. } | StackValue::RefMut { .. } | StackValue::Moved => {
                panic!()
            }
        }
    }

    pub fn eval(&self) -> EvalValue<'eval> {
        match self {
            StackValue::Copyable(primitive_value) => EvalValue::Copyable(*primitive_value),
            StackValue::Owned(boxed_value) => EvalValue::Owned(boxed_value.clone()),
            StackValue::GlobalPure(_) => todo!(),
            StackValue::GlobalRef(value) => EvalValue::GlobalRef(*value),
            StackValue::LocalRef(value) => EvalValue::Owned(value.clone_into_box_dyn().into()),
            StackValue::RefMut { value, .. } => EvalValue::Owned(value.clone_into_box_dyn().into()),
            StackValue::Moved => {
                panic!()
            }
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            StackValue::Moved => todo!(),
            StackValue::Copyable(v) => v.to_bool(),
            StackValue::Owned(_) => todo!(),
            StackValue::GlobalPure(_) => todo!(),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::LocalRef(value) => todo!(),
            StackValue::RefMut { value, owner, gen } => todo!(),
        }
    }

    pub fn into_member(&mut self) -> MemberValue<'eval> {
        match self {
            StackValue::Copyable(primitive_value) => MemberValue::Primitive(*primitive_value),
            StackValue::Owned(boxed_value) => match std::mem::replace(self, StackValue::Moved) {
                StackValue::Owned(boxed_value) => MemberValue::Boxed(boxed_value),
                _ => panic!(),
            },
            StackValue::GlobalPure(_) => todo!(),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::LocalRef { .. } | StackValue::RefMut { .. } | StackValue::Moved => {
                panic!()
            }
        }
    }

    pub(crate) unsafe fn bind(&mut self, binding: Binding, stack_idx: StackIdx) -> Self {
        match binding {
            Binding::Ref => self.bind_ref(),
            Binding::RefMut => self.bind_ref_mut(stack_idx),
            Binding::Move => self.bind_move(),
            Binding::Copy => self.bind_copy(),
        }
    }

    unsafe fn bind_ref(&self) -> Self {
        match self {
            StackValue::Moved => panic!(),
            StackValue::Copyable(_) => panic!(),
            StackValue::Owned(value) => {
                let ptr: *const dyn AnyValueDyn = value.any_ptr();
                StackValue::LocalRef(&*ptr)
            }
            StackValue::GlobalPure(value) => {
                let ptr: *const dyn AnyValueDyn = &**value;
                StackValue::LocalRef(&*ptr)
            }
            StackValue::GlobalRef(value) => StackValue::GlobalRef(*value),
            StackValue::LocalRef(value) => StackValue::LocalRef(*value),
            StackValue::RefMut { value, owner, gen } => todo!(),
        }
    }

    fn bind_copy(&self) -> Self {
        match self {
            StackValue::Moved => todo!(),
            StackValue::Copyable(value) => StackValue::Copyable(*value),
            StackValue::Owned(_) => todo!(),
            StackValue::GlobalPure(_) => todo!(),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::LocalRef(value) => {
                p!(value);
                todo!()
            }
            StackValue::RefMut { value, owner, gen } => todo!(),
        }
    }

    unsafe fn bind_ref_mut(&mut self, stack_idx: StackIdx) -> StackValue<'stack, 'eval> {
        match self {
            StackValue::Copyable(value) => {
                let ptr: *mut dyn AnyValueDyn = value.any_mut();
                StackValue::RefMut {
                    value: &mut *ptr,
                    owner: stack_idx,
                    gen: (),
                }
            }
            StackValue::Owned(value) => {
                let ptr: *mut dyn AnyValueDyn = &mut *value.any_mut_ptr();
                StackValue::RefMut {
                    value: &mut *ptr,
                    owner: stack_idx,
                    gen: (),
                }
            }
            StackValue::Moved
            | StackValue::GlobalPure(_)
            | StackValue::GlobalRef(_)
            | StackValue::LocalRef { .. }
            | StackValue::RefMut { .. } => panic!(),
        }
    }

    unsafe fn pure(&self, stack_idx: StackIdx) -> Self {
        match self {
            StackValue::Copyable(value) => StackValue::Copyable(*value),
            StackValue::Owned(value) => StackValue::LocalRef(&*value.any_ptr()),
            StackValue::GlobalPure(value) => StackValue::GlobalPure(value.clone()),
            StackValue::GlobalRef(value) => StackValue::GlobalRef(*value),
            StackValue::LocalRef { .. } => todo!(),
            StackValue::RefMut { .. } => todo!(),
            StackValue::Moved => todo!(),
        }
    }

    pub(crate) fn bind_move(&mut self) -> Self {
        match self {
            StackValue::Moved => todo!(),
            StackValue::Copyable(value) => StackValue::Copyable(*value),
            StackValue::Owned(_) => std::mem::replace(self, StackValue::Moved),
            StackValue::GlobalPure(_) => todo!(),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::LocalRef { .. } => todo!(),
            StackValue::RefMut { value, owner, gen } => todo!(),
        }
    }

    pub(crate) fn bind_return(&mut self) -> Self {
        match self {
            StackValue::Moved => todo!(),
            StackValue::Copyable(value) => Self::Copyable(*value),
            StackValue::Owned(_) => std::mem::replace(self, StackValue::Moved),
            StackValue::GlobalPure(_) => todo!(),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::LocalRef { .. } => todo!(),
            StackValue::RefMut { value, owner, gen } => todo!(),
        }
    }

    unsafe fn borrow_mut(&mut self, self_stack_idx: StackIdx) -> Self {
        Self::RefMut {
            value: &mut *self.any_mut_ptr(),
            owner: self.owner(self_stack_idx).unwrap(),
            gen: (),
        }
    }

    fn owner(&self, self_stack_idx: StackIdx) -> Option<StackIdx> {
        match self {
            StackValue::Copyable(_) | StackValue::Owned(_) => Some(self_stack_idx),
            StackValue::GlobalRef(_) | StackValue::GlobalPure(_) => None,
            StackValue::LocalRef { .. } => todo!(),
            StackValue::RefMut { owner, .. } => Some(*owner),
            StackValue::Moved => todo!(),
        }
    }

    pub fn any_ref(&self) -> &dyn AnyValueDyn<'eval> {
        {
            match self {
                StackValue::Copyable(value) => match value {
                    CopyableValue::I32(value) => value,
                    CopyableValue::F32(value) => value,
                    CopyableValue::B32(value) => value,
                    CopyableValue::B64(value) => value,
                    CopyableValue::Bool(value) => value,
                    CopyableValue::Void(_) => todo!(),
                    CopyableValue::EnumKind(value) => value,
                },
                StackValue::Owned(value) => value.any_ref(),
                StackValue::GlobalPure(value) => (&**value),
                StackValue::GlobalRef(_) => todo!(),
                StackValue::LocalRef(value) => *value,
                StackValue::RefMut { value, .. } => *value,
                StackValue::Moved => todo!(),
            }
        }
    }

    fn any_mut_ptr(&mut self) -> *mut dyn AnyValueDyn<'eval> {
        {
            match self {
                StackValue::Copyable(value) => match value {
                    CopyableValue::I32(value) => value,
                    CopyableValue::F32(value) => value,
                    CopyableValue::B32(value) => value,
                    CopyableValue::B64(value) => value,
                    CopyableValue::Bool(value) => value,
                    CopyableValue::Void(_) => todo!(),
                    CopyableValue::EnumKind(value) => value,
                },
                StackValue::Owned(value) => value.any_mut_ptr(),
                StackValue::RefMut { value, .. } => *value,
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
            StackValue::Copyable(_) => todo!(),
            StackValue::Owned(_) => todo!(),
            StackValue::GlobalPure(value) => value.downcast_ref(),
            StackValue::GlobalRef(value) => value.downcast_ref(),
            StackValue::LocalRef(value) => value.downcast_ref(),
            StackValue::RefMut { value, .. } => value.downcast_ref(),
        }
    }

    pub fn downcast_mut<T: AnyValue<'eval>>(&mut self) -> &mut T {
        match self {
            StackValue::Moved => todo!(),
            StackValue::Copyable(_) => todo!(),
            StackValue::Owned(_)
            | StackValue::GlobalPure(_)
            | StackValue::GlobalRef(_)
            | StackValue::LocalRef { .. } => {
                panic!()
            }
            StackValue::RefMut { ref mut value, .. } => value.downcast_mut(),
        }
    }

    pub fn downcast_mut_full<T: AnyValue<'eval>>(&mut self) -> (&'stack mut T, StackIdx, ()) {
        match self {
            StackValue::Moved => todo!(),
            StackValue::Copyable(_) => todo!(),
            StackValue::Owned(_)
            | StackValue::GlobalPure(_)
            | StackValue::GlobalRef(_)
            | StackValue::LocalRef { .. } => {
                panic!()
            }
            StackValue::RefMut { value, owner, gen } => {
                let ptr: *mut T = value.downcast_mut();
                (unsafe { &mut *ptr }, *owner, *gen)
            }
        }
    }

    pub fn primitive(&self) -> CopyableValue {
        match self {
            StackValue::Copyable(value) => *value,
            StackValue::RefMut { value, .. } => value.primitive(),
            _ => {
                p!(self);
                panic!("")
            }
        }
    }

    pub fn clone_into_stack(&self) -> StackValue<'stack, 'eval> {
        match self {
            StackValue::Moved => todo!(),
            StackValue::Copyable(_) => todo!(),
            StackValue::Owned(_) => todo!(),
            StackValue::GlobalPure(value) => StackValue::Owned(value.clone_into_box_dyn().into()),
            StackValue::GlobalRef(_) => todo!(),
            StackValue::LocalRef(value) => StackValue::Owned(value.clone_into_box_dyn().into()),
            StackValue::RefMut { value, owner, gen } => todo!(),
        }
    }

    pub(crate) fn snapshot(&self) -> StackValueSnapshot<'eval> {
        match self {
            StackValue::Copyable(value) => StackValueSnapshot::Copyable(*value),
            StackValue::Owned(value) => StackValueSnapshot::Owned(value.clone()),
            StackValue::GlobalPure(value) => StackValueSnapshot::GlobalPure(value.clone()),
            StackValue::GlobalRef(value) => StackValueSnapshot::GlobalRef(*value),
            StackValue::LocalRef(value) => todo!(),
            StackValue::RefMut { value, owner, gen } => {
                p!(value);
                todo!()
            }
            StackValue::Moved => todo!(),
        }
    }

    pub fn static_type_id(&self) -> StaticTypeId {
        self.any_ref().static_type_id_dyn()
    }

    pub fn field(
        self,
        field_idx: usize,
        field_access_contract: EagerContract,
    ) -> StackValue<'stack, 'eval> {
        match self {
            StackValue::Moved => todo!(),
            StackValue::Copyable(_) => todo!(),
            StackValue::Owned(boxed_value) => {
                let mut value: VirtualTy = boxed_value.take().unwrap();
                value.take_field(field_idx)
            }
            StackValue::GlobalPure(_) => todo!(),
            StackValue::GlobalRef(value) => {
                let value: &VirtualTy = value.downcast_ref();
                value.eager_field(field_idx, field_access_contract)
            }
            StackValue::LocalRef(value) => {
                let value: &VirtualTy = value.downcast_ref();
                value.eager_field(field_idx, field_access_contract)
            }
            StackValue::RefMut { value, owner, gen } => {
                let virtual_value: &mut VirtualTy = value.downcast_mut();
                virtual_value.field_mut(field_idx, field_access_contract, owner)
            }
        }
    }
}
