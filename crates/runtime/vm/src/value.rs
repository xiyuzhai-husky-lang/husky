mod any;
mod copyable;
mod enum_kind;
mod eval;
mod member;
mod owned;
mod ref_;
mod xml;

pub use any::*;
pub use copyable::*;
pub use enum_kind::*;
pub use eval::{EvalResult, EvalValue};
pub use member::*;
pub use owned::*;
pub use ref_::*;
pub use xml::*;

use crate::*;
use print_utils::{msg_once, p};
use std::fmt::Write;
use std::sync::Arc;
use word::CustomIdentifier;

// the primary concerns are safety and stability
// this whole vm thing will be replaced by JIT for fast evaluation purposes
// so we don't need to worry too much about speed here
pub enum TempValue<'temp, 'eval: 'temp> {
    Moved,
    Copyable(CopyableValue),
    EvalOwned(OwnedValue<'eval, 'eval>),
    TempOwned(OwnedValue<'temp, 'eval>),
    EvalPure(Arc<dyn AnyValueDyn<'eval> + 'eval>),
    EvalRef(&'eval (dyn AnyValueDyn<'eval> + 'eval)),
    FullyOwnedRef(&'temp (dyn AnyValueDyn<'eval> + 'eval)),
    PartiallyOwnedRef(&'temp (dyn AnyValueDyn<'eval> + 'temp)),
    CopyableOrFullyOwnedMut {
        value: &'temp mut (dyn AnyValueDyn<'eval> + 'eval),
        owner: VMStackIdx,
        gen: MutRefGenerator,
    },
    PartiallyOwnedMut {
        value: &'temp mut (dyn AnyValueDyn<'eval> + 'temp),
        owner: VMStackIdx,
        gen: MutRefGenerator,
    },
}

pub type MutRefGenerator = ();

impl<'vm, 'eval: 'vm> std::fmt::Debug for TempValue<'vm, 'eval> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TempValue::Copyable(arg0) => {
                f.write_str("Primitive ")?;
                arg0.fmt(f)
            }
            TempValue::EvalOwned(arg0) => f.debug_tuple("Boxed").field(arg0).finish(),
            TempValue::EvalPure(arg0) => f.debug_tuple("GlobalPure").field(arg0).finish(),
            TempValue::EvalRef(arg0) => f.debug_tuple("EvalRef").field(arg0).finish(),
            TempValue::FullyOwnedRef(value) => f.debug_tuple("Ref").field(value).finish(),
            TempValue::CopyableOrFullyOwnedMut { value, .. } => {
                f.debug_tuple("MutRef").field(value).finish()
            }
            TempValue::Moved => f.write_str("Taken"),
            TempValue::TempOwned(_) => todo!(),
            TempValue::PartiallyOwnedRef(_) => todo!(),
            TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }
}

impl<'vm, 'eval: 'vm> TempValue<'vm, 'eval> {
    pub fn print_short(&self) -> String {
        let mut result = String::new();
        match self {
            TempValue::Moved => result.push_str("Moved"),
            TempValue::Copyable(value) => {
                result.push_str("Primitive ");
                result.push_str(&value.any_ref().print_short())
            }
            TempValue::EvalOwned(value) => {
                result.push_str("Boxed ");
                result.push_str(&value.any_ref().print_short())
            }
            TempValue::EvalPure(value) => {
                result.push_str("GlobalPure ");
                result.push_str(&value.print_short())
            }
            TempValue::EvalRef(value) => {
                result.push_str("EvalRef ");
                result.push_str(&value.print_short());
            }
            TempValue::FullyOwnedRef(value) => {
                result.push_str("TempRef ");
                result.push_str(&value.print_short());
            }
            TempValue::CopyableOrFullyOwnedMut { value, owner, gen } => {
                result.push_str("TempRefMut ");
                result.push_str(&value.print_short());
                write!(result, " Owner({:?}) ", owner);
            }
            TempValue::TempOwned(_) => todo!(),
            TempValue::PartiallyOwnedRef(_) => todo!(),
            TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
        result
    }

    pub fn to_json_value(self) -> serde_json::value::Value {
        match self {
            TempValue::Moved => todo!(),
            TempValue::Copyable(_) => todo!(),
            TempValue::EvalOwned(_) => todo!(),
            TempValue::EvalPure(_) => todo!(),
            TempValue::EvalRef(_) => todo!(),
            TempValue::FullyOwnedRef(value) => value.get_json_value_dyn(),
            TempValue::CopyableOrFullyOwnedMut { value, owner, gen } => todo!(),
            TempValue::TempOwned(_) => todo!(),
            TempValue::PartiallyOwnedRef(_) => todo!(),
            TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }
}

impl<'vm, 'eval: 'vm> From<CopyableValue> for TempValue<'vm, 'eval> {
    fn from(value: CopyableValue) -> Self {
        TempValue::Copyable(value)
    }
}

impl<'vm, 'eval: 'vm> From<&CopyableValue> for TempValue<'vm, 'eval> {
    fn from(value: &CopyableValue) -> Self {
        TempValue::Copyable(*value)
    }
}

impl<'vm, 'eval: 'vm> TempValue<'vm, 'eval> {
    pub fn from_eval(eval_value: EvalValue<'eval>) -> VMRuntimeResult<Self> {
        Ok(match eval_value {
            EvalValue::Copyable(value) => Self::Copyable(value),
            EvalValue::Owned(_) => todo!(),
            EvalValue::GlobalPure(value) => TempValue::EvalPure(value),
            EvalValue::EvalRef(value) => Self::EvalRef(value),
            EvalValue::Undefined => todo!(),
        })
    }

    pub fn into_eval(self) -> EvalValue<'eval> {
        match self {
            TempValue::Copyable(copyable_value) => EvalValue::Copyable(copyable_value),
            TempValue::EvalOwned(boxed_value) => EvalValue::Owned(boxed_value),
            TempValue::EvalPure(_) => todo!(),
            TempValue::EvalRef(value) => EvalValue::EvalRef(value),
            TempValue::FullyOwnedRef { .. }
            | TempValue::CopyableOrFullyOwnedMut { .. }
            | TempValue::Moved => {
                panic!()
            }
            TempValue::TempOwned(_) => todo!(),
            TempValue::PartiallyOwnedRef(_) => todo!(),
            TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    pub fn eval(&self) -> EvalValue<'eval> {
        match self {
            TempValue::Copyable(primitive_value) => EvalValue::Copyable(*primitive_value),
            TempValue::EvalOwned(boxed_value) => EvalValue::Owned(boxed_value.clone()),
            TempValue::EvalPure(_) => todo!(),
            TempValue::EvalRef(value) => EvalValue::EvalRef(*value),
            TempValue::FullyOwnedRef(value) => EvalValue::Owned(value.clone_into_box_dyn().into()),
            TempValue::CopyableOrFullyOwnedMut { value, .. } => {
                EvalValue::Owned(value.clone_into_box_dyn().into())
            }
            TempValue::Moved => {
                panic!()
            }
            TempValue::TempOwned(_) => todo!(),
            TempValue::PartiallyOwnedRef(_) => todo!(),
            TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            TempValue::Moved => todo!(),
            TempValue::Copyable(v) => v.to_bool(),
            TempValue::EvalOwned(_) => todo!(),
            TempValue::EvalPure(_) => todo!(),
            TempValue::EvalRef(_) => todo!(),
            TempValue::FullyOwnedRef(value) => todo!(),
            TempValue::CopyableOrFullyOwnedMut { value, owner, gen } => todo!(),
            TempValue::TempOwned(_) => todo!(),
            TempValue::PartiallyOwnedRef(_) => todo!(),
            TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    pub fn into_member(&mut self) -> MemberValue<'eval> {
        match self {
            TempValue::Copyable(primitive_value) => MemberValue::Copyable(*primitive_value),
            TempValue::EvalOwned(boxed_value) => match std::mem::replace(self, TempValue::Moved) {
                TempValue::EvalOwned(boxed_value) => MemberValue::Boxed(boxed_value),
                _ => panic!(),
            },
            TempValue::EvalPure(_) => todo!(),
            TempValue::EvalRef(_) => todo!(),
            TempValue::FullyOwnedRef { .. }
            | TempValue::CopyableOrFullyOwnedMut { .. }
            | TempValue::Moved => {
                panic!()
            }
            TempValue::TempOwned(_) => todo!(),
            TempValue::PartiallyOwnedRef(_) => todo!(),
            TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    pub(crate) unsafe fn bind(&mut self, binding: Binding, stack_idx: VMStackIdx) -> Self {
        match binding {
            Binding::Ref => self.bind_ref(),
            Binding::RefMut => self.bind_ref_mut(stack_idx),
            Binding::Move => self.bind_move(),
            Binding::Copy => self.bind_copy(),
        }
    }

    unsafe fn bind_ref(&self) -> Self {
        match self {
            TempValue::Moved => panic!(),
            TempValue::Copyable(_) => panic!(),
            TempValue::EvalOwned(value) => {
                let ptr: *const dyn AnyValueDyn = value.any_ptr();
                TempValue::FullyOwnedRef(&*ptr)
            }
            TempValue::EvalPure(value) => {
                let ptr: *const dyn AnyValueDyn = &**value;
                TempValue::FullyOwnedRef(&*ptr)
            }
            TempValue::EvalRef(value) => TempValue::EvalRef(*value),
            TempValue::FullyOwnedRef(value) => TempValue::FullyOwnedRef(*value),
            TempValue::CopyableOrFullyOwnedMut { value, owner, gen } => todo!(),
            TempValue::TempOwned(_) => todo!(),
            TempValue::PartiallyOwnedRef(_) => todo!(),
            TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    fn bind_copy(&self) -> Self {
        match self {
            TempValue::Moved => todo!(),
            TempValue::Copyable(value) => TempValue::Copyable(*value),
            TempValue::EvalOwned(_) => todo!(),
            TempValue::EvalPure(_) => todo!(),
            TempValue::EvalRef(_) => todo!(),
            TempValue::FullyOwnedRef(value) => {
                p!(value);
                todo!()
            }
            TempValue::CopyableOrFullyOwnedMut { value, owner, gen } => todo!(),
            TempValue::TempOwned(_) => todo!(),
            TempValue::PartiallyOwnedRef(_) => todo!(),
            TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    unsafe fn bind_ref_mut(&mut self, stack_idx: VMStackIdx) -> TempValue<'vm, 'eval> {
        match self {
            TempValue::Copyable(value) => {
                let ptr: *mut dyn AnyValueDyn<'eval> = value.any_mut();
                TempValue::CopyableOrFullyOwnedMut {
                    value: &mut *ptr,
                    owner: stack_idx,
                    gen: (),
                }
            }
            TempValue::EvalOwned(value) => {
                let ptr: *mut dyn AnyValueDyn = &mut *value.any_mut_ptr();
                TempValue::CopyableOrFullyOwnedMut {
                    value: &mut *ptr,
                    owner: stack_idx,
                    gen: (),
                }
            }
            TempValue::Moved
            | TempValue::EvalPure(_)
            | TempValue::EvalRef(_)
            | TempValue::FullyOwnedRef { .. }
            | TempValue::CopyableOrFullyOwnedMut { .. } => panic!(),
            TempValue::TempOwned(_) => todo!(),
            TempValue::PartiallyOwnedRef(_) => todo!(),
            TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    unsafe fn pure(&self, stack_idx: VMStackIdx) -> Self {
        match self {
            TempValue::Copyable(value) => TempValue::Copyable(*value),
            TempValue::EvalOwned(value) => TempValue::FullyOwnedRef(&*value.any_ptr()),
            TempValue::EvalPure(value) => TempValue::EvalPure(value.clone()),
            TempValue::EvalRef(value) => TempValue::EvalRef(*value),
            TempValue::FullyOwnedRef { .. } => todo!(),
            TempValue::CopyableOrFullyOwnedMut { .. } => todo!(),
            TempValue::Moved => todo!(),
            TempValue::TempOwned(_) => todo!(),
            TempValue::PartiallyOwnedRef(_) => todo!(),
            TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    pub(crate) fn bind_move(&mut self) -> Self {
        match self {
            TempValue::Moved => todo!(),
            TempValue::Copyable(value) => TempValue::Copyable(*value),
            TempValue::EvalOwned(_) => std::mem::replace(self, TempValue::Moved),
            TempValue::EvalPure(_) => todo!(),
            TempValue::EvalRef(_) => todo!(),
            TempValue::FullyOwnedRef { .. } => todo!(),
            TempValue::CopyableOrFullyOwnedMut { value, owner, gen } => todo!(),
            TempValue::TempOwned(_) => todo!(),
            TempValue::PartiallyOwnedRef(_) => todo!(),
            TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    pub(crate) fn bind_return(&mut self) -> Self {
        match self {
            TempValue::Moved => todo!(),
            TempValue::Copyable(value) => Self::Copyable(*value),
            TempValue::EvalOwned(_) => std::mem::replace(self, TempValue::Moved),
            TempValue::EvalPure(_) => todo!(),
            TempValue::EvalRef(_) => todo!(),
            TempValue::FullyOwnedRef { .. } => todo!(),
            TempValue::CopyableOrFullyOwnedMut { value, owner, gen } => todo!(),
            TempValue::TempOwned(_) => todo!(),
            TempValue::PartiallyOwnedRef(_) => todo!(),
            TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    unsafe fn borrow_mut(&mut self, self_stack_idx: VMStackIdx) -> Self {
        Self::CopyableOrFullyOwnedMut {
            value: &mut *self.any_mut_ptr(),
            owner: self.owner(self_stack_idx).unwrap(),
            gen: (),
        }
    }

    fn owner(&self, self_stack_idx: VMStackIdx) -> Option<VMStackIdx> {
        match self {
            TempValue::Copyable(_) | TempValue::EvalOwned(_) => Some(self_stack_idx),
            TempValue::EvalRef(_) | TempValue::EvalPure(_) => None,
            TempValue::FullyOwnedRef { .. } => todo!(),
            TempValue::CopyableOrFullyOwnedMut { owner, .. } => Some(*owner),
            TempValue::Moved => todo!(),
            TempValue::TempOwned(_) => todo!(),
            TempValue::PartiallyOwnedRef(_) => todo!(),
            TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    pub fn any_ref(&self) -> &dyn AnyValueDyn<'eval> {
        {
            match self {
                TempValue::Copyable(value) => match value {
                    CopyableValue::I32(value) => value,
                    CopyableValue::F32(value) => value,
                    CopyableValue::B32(value) => value,
                    CopyableValue::B64(value) => value,
                    CopyableValue::Bool(value) => value,
                    CopyableValue::Void(_) => todo!(),
                    CopyableValue::EnumKind(value) => value,
                },
                TempValue::EvalOwned(value) => value.any_ref(),
                TempValue::EvalPure(value) => (&**value),
                TempValue::EvalRef(_) => todo!(),
                TempValue::FullyOwnedRef(value) => *value,
                TempValue::CopyableOrFullyOwnedMut { value, .. } => *value,
                TempValue::Moved => todo!(),
                TempValue::TempOwned(_) => todo!(),
                TempValue::PartiallyOwnedRef(_) => todo!(),
                TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
            }
        }
    }

    fn any_mut_ptr(&mut self) -> *mut (dyn AnyValueDyn<'eval> + 'eval) {
        {
            match self {
                TempValue::Copyable(value) => match value {
                    CopyableValue::I32(value) => value,
                    CopyableValue::F32(value) => value,
                    CopyableValue::B32(value) => value,
                    CopyableValue::B64(value) => value,
                    CopyableValue::Bool(value) => value,
                    CopyableValue::Void(_) => todo!(),
                    CopyableValue::EnumKind(value) => value,
                },
                TempValue::EvalOwned(value) => value.any_mut_ptr(),
                TempValue::CopyableOrFullyOwnedMut { value, .. } => *value,
                TempValue::FullyOwnedRef { .. } => {
                    panic!("TempRef cannot be mutated, this is a bug.")
                }
                TempValue::EvalPure(_) => panic!("GlobalPure cannot be mutated, this is a bug."),
                TempValue::EvalRef(_) => panic!("EvalRef cannot be mutated, this is a bug."),
                TempValue::Moved => panic!("Move cannot be mutated, this is a bug."),
                TempValue::TempOwned(_) => todo!(),
                TempValue::PartiallyOwnedRef(_) => todo!(),
                TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
            }
        }
    }

    pub fn downcast_ref<T: AnyValue<'eval>>(&self) -> &T {
        match self {
            TempValue::Moved => todo!(),
            TempValue::Copyable(_) => todo!(),
            TempValue::EvalOwned(_) => todo!(),
            TempValue::EvalPure(value) => value.downcast_ref(),
            TempValue::EvalRef(value) => value.downcast_ref(),
            TempValue::FullyOwnedRef(value) => value.downcast_ref(),
            TempValue::CopyableOrFullyOwnedMut { value, .. } => value.downcast_ref(),
            TempValue::TempOwned(_) => todo!(),
            TempValue::PartiallyOwnedRef(_) => todo!(),
            TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    pub fn downcast_eval_ref<T: AnyValue<'eval>>(&self) -> &'eval T {
        match self {
            TempValue::Moved => todo!(),
            TempValue::Copyable(_) => todo!(),
            TempValue::EvalOwned(_) => todo!(),
            TempValue::EvalPure(value) => panic!(),
            TempValue::EvalRef(value) => value.downcast_ref(),
            TempValue::FullyOwnedRef(value) => panic!(),
            TempValue::CopyableOrFullyOwnedMut { value, .. } => panic!(),
            TempValue::TempOwned(_) => todo!(),
            TempValue::PartiallyOwnedRef(_) => todo!(),
            TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    pub fn downcast_mut<T: AnyValue<'eval>>(&mut self) -> &mut T {
        match self {
            TempValue::Moved => todo!(),
            TempValue::Copyable(_) => todo!(),
            TempValue::EvalOwned(_)
            | TempValue::EvalPure(_)
            | TempValue::EvalRef(_)
            | TempValue::FullyOwnedRef { .. } => {
                panic!()
            }
            TempValue::CopyableOrFullyOwnedMut { ref mut value, .. } => value.downcast_mut(),
            TempValue::TempOwned(_) => todo!(),
            TempValue::PartiallyOwnedRef(_) => todo!(),
            TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    pub fn downcast_mut_full<T: AnyValue<'eval>>(&mut self) -> (&'vm mut T, VMStackIdx, ()) {
        match self {
            TempValue::Moved => todo!(),
            TempValue::Copyable(_) => todo!(),
            TempValue::EvalOwned(_)
            | TempValue::EvalPure(_)
            | TempValue::EvalRef(_)
            | TempValue::FullyOwnedRef { .. } => {
                panic!()
            }
            TempValue::CopyableOrFullyOwnedMut { value, owner, gen } => {
                let ptr: *mut T = value.downcast_mut();
                (unsafe { &mut *ptr }, *owner, *gen)
            }
            TempValue::TempOwned(_) => todo!(),
            TempValue::PartiallyOwnedRef(_) => todo!(),
            TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    pub fn take_copyable(&self) -> CopyableValue {
        match self {
            TempValue::Copyable(value) => *value,
            TempValue::CopyableOrFullyOwnedMut { value, .. } => value.take_copyable(),
            _ => {
                p!(self);
                panic!("")
            }
        }
    }

    pub fn clone_into_stack(&self) -> TempValue<'vm, 'eval> {
        match self {
            TempValue::Moved => todo!(),
            TempValue::Copyable(_) => todo!(),
            TempValue::EvalOwned(_) => todo!(),
            TempValue::EvalPure(value) => TempValue::EvalOwned(value.clone_into_box_dyn().into()),
            TempValue::EvalRef(_) => todo!(),
            TempValue::FullyOwnedRef(value) => {
                TempValue::EvalOwned(value.clone_into_box_dyn().into())
            }
            TempValue::CopyableOrFullyOwnedMut { value, owner, gen } => todo!(),
            TempValue::TempOwned(_) => todo!(),
            TempValue::PartiallyOwnedRef(_) => todo!(),
            TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    pub(crate) fn snapshot(&self) -> StackValueSnapshot<'eval> {
        match self {
            TempValue::Copyable(value) => StackValueSnapshot::Copyable(*value),
            TempValue::EvalOwned(value) => StackValueSnapshot::Owned(value.clone()),
            TempValue::EvalPure(value) => StackValueSnapshot::GlobalPure(value.clone()),
            TempValue::EvalRef(value) => StackValueSnapshot::EvalRef(*value),
            TempValue::FullyOwnedRef(value) => {
                StackValueSnapshot::FullyOwnedRef(value.clone_into_arc_dyn())
            }
            TempValue::CopyableOrFullyOwnedMut { value, owner, gen } => {
                p!(value);
                todo!()
            }
            TempValue::Moved => todo!(),
            TempValue::TempOwned(_) => todo!(),
            TempValue::PartiallyOwnedRef(_) => todo!(),
            TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    pub fn static_type_id(&self) -> StaticTypeId {
        self.any_ref().static_type_id_dyn()
    }

    pub fn field(self, field_idx: usize, field_binding: Binding) -> TempValue<'vm, 'eval> {
        match self {
            TempValue::Moved => todo!(),
            TempValue::Copyable(_) => todo!(),
            TempValue::EvalOwned(boxed_value) => {
                let mut value: VirtualTy = boxed_value.take().unwrap();
                value.take_field(field_idx)
            }
            TempValue::EvalPure(_) => todo!(),
            TempValue::EvalRef(value) => {
                let value: &VirtualTy = value.downcast_ref();
                value.access_field(field_idx, field_binding)
            }
            TempValue::FullyOwnedRef(value) => {
                let value: &VirtualTy = value.downcast_ref();
                value.access_field(field_idx, field_binding)
            }
            TempValue::CopyableOrFullyOwnedMut { value, owner, gen } => {
                let virtual_value: &mut VirtualTy = value.downcast_mut();
                virtual_value.field_mut(field_idx, field_binding, owner)
            }
            TempValue::TempOwned(_) => todo!(),
            TempValue::PartiallyOwnedRef(_) => todo!(),
            TempValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }
}
