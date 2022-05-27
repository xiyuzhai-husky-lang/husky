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

// the primary goal is to make sure that debugging is easy
// guaranteed memory safety
pub enum VMValue<'vm, 'eval: 'vm> {
    Moved,
    Copyable(CopyableValue),
    FullyOwned(OwnedValue<'eval, 'eval>),
    PartiallyOwned(OwnedValue<'vm, 'eval>),
    EvalPure(Arc<dyn AnyValueDyn<'eval> + 'eval>),
    EvalRef(&'eval (dyn AnyValueDyn<'eval> + 'eval)),
    FullyOwnedRef(&'vm (dyn AnyValueDyn<'eval> + 'eval)),
    PartiallyOwnedRef(&'vm (dyn AnyValueDyn<'eval> + 'vm)),
    CopyableMut {
        value: &'vm mut CopyableValue,
        owner: VMStackIdx,
        gen: MutRefGenerator,
    },
    FullyOwnedMut {
        value: &'vm mut (dyn AnyValueDyn<'eval> + 'eval),
        owner: VMStackIdx,
        gen: MutRefGenerator,
    },
    PartiallyOwnedMut {
        value: &'vm mut (dyn AnyValueDyn<'eval> + 'vm),
        owner: VMStackIdx,
        gen: MutRefGenerator,
    },
}

pub type MutRefGenerator = ();

impl<'vm, 'eval: 'vm> std::fmt::Debug for VMValue<'vm, 'eval> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VMValue::Copyable(arg0) => {
                f.write_str("Primitive ")?;
                arg0.fmt(f)
            }
            VMValue::FullyOwned(arg0) => f.debug_tuple("Boxed").field(arg0).finish(),
            VMValue::EvalPure(arg0) => f.debug_tuple("GlobalPure").field(arg0).finish(),
            VMValue::EvalRef(arg0) => f.debug_tuple("GlobalRef").field(arg0).finish(),
            VMValue::FullyOwnedRef(value) => f.debug_tuple("Ref").field(value).finish(),
            VMValue::FullyOwnedMut { value, .. } => f.debug_tuple("MutRef").field(value).finish(),
            VMValue::Moved => f.write_str("Taken"),
            VMValue::PartiallyOwned(_) => todo!(),
            VMValue::PartiallyOwnedRef(_) => todo!(),
            VMValue::CopyableMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }
}

impl<'vm, 'eval: 'vm> VMValue<'vm, 'eval> {
    pub fn print_short(&self) -> String {
        let mut result = String::new();
        match self {
            VMValue::Moved => result.push_str("Moved"),
            VMValue::Copyable(value) => {
                result.push_str("Primitive ");
                result.push_str(&value.any_ref().print_short())
            }
            VMValue::FullyOwned(value) => {
                result.push_str("Boxed ");
                result.push_str(&value.any_ref().print_short())
            }
            VMValue::EvalPure(value) => {
                result.push_str("GlobalPure ");
                result.push_str(&value.print_short())
            }
            VMValue::EvalRef(value) => {
                result.push_str("GlobalRef ");
                result.push_str(&value.print_short());
            }
            VMValue::FullyOwnedRef(value) => {
                result.push_str("LocalRef ");
                result.push_str(&value.print_short());
            }
            VMValue::FullyOwnedMut { value, owner, gen } => {
                result.push_str("LocalRefMut ");
                result.push_str(&value.print_short());
                write!(result, " Owner({:?}) ", owner);
            }
            VMValue::PartiallyOwned(_) => todo!(),
            VMValue::PartiallyOwnedRef(_) => todo!(),
            VMValue::CopyableMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
        result
    }

    pub fn to_json_value(self) -> serde_json::value::Value {
        match self {
            VMValue::Moved => todo!(),
            VMValue::Copyable(_) => todo!(),
            VMValue::FullyOwned(_) => todo!(),
            VMValue::EvalPure(_) => todo!(),
            VMValue::EvalRef(_) => todo!(),
            VMValue::FullyOwnedRef(value) => value.get_json_value_dyn(),
            VMValue::FullyOwnedMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwned(_) => todo!(),
            VMValue::PartiallyOwnedRef(_) => todo!(),
            VMValue::CopyableMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }
}

impl<'vm, 'eval: 'vm> From<CopyableValue> for VMValue<'vm, 'eval> {
    fn from(value: CopyableValue) -> Self {
        VMValue::Copyable(value)
    }
}

impl<'vm, 'eval: 'vm> From<&CopyableValue> for VMValue<'vm, 'eval> {
    fn from(value: &CopyableValue) -> Self {
        VMValue::Copyable(*value)
    }
}

impl<'vm, 'eval: 'vm> VMValue<'vm, 'eval> {
    pub fn from_eval(eval_value: EvalValue<'eval>) -> VMRuntimeResult<Self> {
        Ok(match eval_value {
            EvalValue::Copyable(value) => Self::Copyable(value),
            EvalValue::Owned(_) => todo!(),
            EvalValue::GlobalPure(value) => VMValue::EvalPure(value),
            EvalValue::GlobalRef(value) => Self::EvalRef(value),
            EvalValue::Undefined => todo!(),
        })
    }

    pub fn into_eval(self) -> EvalValue<'eval> {
        match self {
            VMValue::Copyable(copyable_value) => EvalValue::Copyable(copyable_value),
            VMValue::FullyOwned(boxed_value) => EvalValue::Owned(boxed_value),
            VMValue::EvalPure(_) => todo!(),
            VMValue::EvalRef(value) => EvalValue::GlobalRef(value),
            VMValue::FullyOwnedRef { .. } | VMValue::FullyOwnedMut { .. } | VMValue::Moved => {
                panic!()
            }
            VMValue::PartiallyOwned(_) => todo!(),
            VMValue::PartiallyOwnedRef(_) => todo!(),
            VMValue::CopyableMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    pub fn eval(&self) -> EvalValue<'eval> {
        match self {
            VMValue::Copyable(primitive_value) => EvalValue::Copyable(*primitive_value),
            VMValue::FullyOwned(boxed_value) => EvalValue::Owned(boxed_value.clone()),
            VMValue::EvalPure(_) => todo!(),
            VMValue::EvalRef(value) => EvalValue::GlobalRef(*value),
            VMValue::FullyOwnedRef(value) => EvalValue::Owned(value.clone_into_box_dyn().into()),
            VMValue::FullyOwnedMut { value, .. } => {
                EvalValue::Owned(value.clone_into_box_dyn().into())
            }
            VMValue::Moved => {
                panic!()
            }
            VMValue::PartiallyOwned(_) => todo!(),
            VMValue::PartiallyOwnedRef(_) => todo!(),
            VMValue::CopyableMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            VMValue::Moved => todo!(),
            VMValue::Copyable(v) => v.to_bool(),
            VMValue::FullyOwned(_) => todo!(),
            VMValue::EvalPure(_) => todo!(),
            VMValue::EvalRef(_) => todo!(),
            VMValue::FullyOwnedRef(value) => todo!(),
            VMValue::FullyOwnedMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwned(_) => todo!(),
            VMValue::PartiallyOwnedRef(_) => todo!(),
            VMValue::CopyableMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    pub fn into_member(&mut self) -> MemberValue<'eval> {
        match self {
            VMValue::Copyable(primitive_value) => MemberValue::Copyable(*primitive_value),
            VMValue::FullyOwned(boxed_value) => match std::mem::replace(self, VMValue::Moved) {
                VMValue::FullyOwned(boxed_value) => MemberValue::Boxed(boxed_value),
                _ => panic!(),
            },
            VMValue::EvalPure(_) => todo!(),
            VMValue::EvalRef(_) => todo!(),
            VMValue::FullyOwnedRef { .. } | VMValue::FullyOwnedMut { .. } | VMValue::Moved => {
                panic!()
            }
            VMValue::PartiallyOwned(_) => todo!(),
            VMValue::PartiallyOwnedRef(_) => todo!(),
            VMValue::CopyableMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
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
            VMValue::Moved => panic!(),
            VMValue::Copyable(_) => panic!(),
            VMValue::FullyOwned(value) => {
                let ptr: *const dyn AnyValueDyn = value.any_ptr();
                VMValue::FullyOwnedRef(&*ptr)
            }
            VMValue::EvalPure(value) => {
                let ptr: *const dyn AnyValueDyn = &**value;
                VMValue::FullyOwnedRef(&*ptr)
            }
            VMValue::EvalRef(value) => VMValue::EvalRef(*value),
            VMValue::FullyOwnedRef(value) => VMValue::FullyOwnedRef(*value),
            VMValue::FullyOwnedMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwned(_) => todo!(),
            VMValue::PartiallyOwnedRef(_) => todo!(),
            VMValue::CopyableMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    fn bind_copy(&self) -> Self {
        match self {
            VMValue::Moved => todo!(),
            VMValue::Copyable(value) => VMValue::Copyable(*value),
            VMValue::FullyOwned(_) => todo!(),
            VMValue::EvalPure(_) => todo!(),
            VMValue::EvalRef(_) => todo!(),
            VMValue::FullyOwnedRef(value) => {
                p!(value);
                todo!()
            }
            VMValue::FullyOwnedMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwned(_) => todo!(),
            VMValue::PartiallyOwnedRef(_) => todo!(),
            VMValue::CopyableMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    unsafe fn bind_ref_mut(&mut self, stack_idx: VMStackIdx) -> VMValue<'vm, 'eval> {
        match self {
            VMValue::Copyable(value) => {
                let ptr: *mut dyn AnyValueDyn<'eval> = value.any_mut();
                VMValue::FullyOwnedMut {
                    value: &mut *ptr,
                    owner: stack_idx,
                    gen: (),
                }
            }
            VMValue::FullyOwned(value) => {
                let ptr: *mut dyn AnyValueDyn = &mut *value.any_mut_ptr();
                VMValue::FullyOwnedMut {
                    value: &mut *ptr,
                    owner: stack_idx,
                    gen: (),
                }
            }
            VMValue::Moved
            | VMValue::EvalPure(_)
            | VMValue::EvalRef(_)
            | VMValue::FullyOwnedRef { .. }
            | VMValue::FullyOwnedMut { .. } => panic!(),
            VMValue::PartiallyOwned(_) => todo!(),
            VMValue::PartiallyOwnedRef(_) => todo!(),
            VMValue::CopyableMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    unsafe fn pure(&self, stack_idx: VMStackIdx) -> Self {
        match self {
            VMValue::Copyable(value) => VMValue::Copyable(*value),
            VMValue::FullyOwned(value) => VMValue::FullyOwnedRef(&*value.any_ptr()),
            VMValue::EvalPure(value) => VMValue::EvalPure(value.clone()),
            VMValue::EvalRef(value) => VMValue::EvalRef(*value),
            VMValue::FullyOwnedRef { .. } => todo!(),
            VMValue::FullyOwnedMut { .. } => todo!(),
            VMValue::Moved => todo!(),
            VMValue::PartiallyOwned(_) => todo!(),
            VMValue::PartiallyOwnedRef(_) => todo!(),
            VMValue::CopyableMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    pub(crate) fn bind_move(&mut self) -> Self {
        match self {
            VMValue::Moved => todo!(),
            VMValue::Copyable(value) => VMValue::Copyable(*value),
            VMValue::FullyOwned(_) => std::mem::replace(self, VMValue::Moved),
            VMValue::EvalPure(_) => todo!(),
            VMValue::EvalRef(_) => todo!(),
            VMValue::FullyOwnedRef { .. } => todo!(),
            VMValue::FullyOwnedMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwned(_) => todo!(),
            VMValue::PartiallyOwnedRef(_) => todo!(),
            VMValue::CopyableMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    pub(crate) fn bind_return(&mut self) -> Self {
        match self {
            VMValue::Moved => todo!(),
            VMValue::Copyable(value) => Self::Copyable(*value),
            VMValue::FullyOwned(_) => std::mem::replace(self, VMValue::Moved),
            VMValue::EvalPure(_) => todo!(),
            VMValue::EvalRef(_) => todo!(),
            VMValue::FullyOwnedRef { .. } => todo!(),
            VMValue::FullyOwnedMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwned(_) => todo!(),
            VMValue::PartiallyOwnedRef(_) => todo!(),
            VMValue::CopyableMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    unsafe fn borrow_mut(&mut self, self_stack_idx: VMStackIdx) -> Self {
        Self::FullyOwnedMut {
            value: &mut *self.any_mut_ptr(),
            owner: self.owner(self_stack_idx).unwrap(),
            gen: (),
        }
    }

    fn owner(&self, self_stack_idx: VMStackIdx) -> Option<VMStackIdx> {
        match self {
            VMValue::Copyable(_) | VMValue::FullyOwned(_) => Some(self_stack_idx),
            VMValue::EvalRef(_) | VMValue::EvalPure(_) => None,
            VMValue::FullyOwnedRef { .. } => todo!(),
            VMValue::FullyOwnedMut { owner, .. } => Some(*owner),
            VMValue::Moved => todo!(),
            VMValue::PartiallyOwned(_) => todo!(),
            VMValue::PartiallyOwnedRef(_) => todo!(),
            VMValue::CopyableMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    pub fn any_ref(&self) -> &dyn AnyValueDyn<'eval> {
        {
            match self {
                VMValue::Copyable(value) => match value {
                    CopyableValue::I32(value) => value,
                    CopyableValue::F32(value) => value,
                    CopyableValue::B32(value) => value,
                    CopyableValue::B64(value) => value,
                    CopyableValue::Bool(value) => value,
                    CopyableValue::Void(_) => todo!(),
                    CopyableValue::EnumKind(value) => value,
                },
                VMValue::FullyOwned(value) => value.any_ref(),
                VMValue::EvalPure(value) => (&**value),
                VMValue::EvalRef(_) => todo!(),
                VMValue::FullyOwnedRef(value) => *value,
                VMValue::FullyOwnedMut { value, .. } => *value,
                VMValue::Moved => todo!(),
                VMValue::PartiallyOwned(_) => todo!(),
                VMValue::PartiallyOwnedRef(_) => todo!(),
                VMValue::CopyableMut { value, owner, gen } => todo!(),
                VMValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
            }
        }
    }

    fn any_mut_ptr(&mut self) -> *mut (dyn AnyValueDyn<'eval> + 'eval) {
        {
            match self {
                VMValue::Copyable(value) => match value {
                    CopyableValue::I32(value) => value,
                    CopyableValue::F32(value) => value,
                    CopyableValue::B32(value) => value,
                    CopyableValue::B64(value) => value,
                    CopyableValue::Bool(value) => value,
                    CopyableValue::Void(_) => todo!(),
                    CopyableValue::EnumKind(value) => value,
                },
                VMValue::FullyOwned(value) => value.any_mut_ptr(),
                VMValue::FullyOwnedMut { value, .. } => *value,
                VMValue::FullyOwnedRef { .. } => {
                    panic!("LocalRef cannot be mutated, this is a bug.")
                }
                VMValue::EvalPure(_) => panic!("GlobalPure cannot be mutated, this is a bug."),
                VMValue::EvalRef(_) => panic!("GlobalRef cannot be mutated, this is a bug."),
                VMValue::Moved => panic!("Move cannot be mutated, this is a bug."),
                VMValue::PartiallyOwned(_) => todo!(),
                VMValue::PartiallyOwnedRef(_) => todo!(),
                VMValue::CopyableMut { value, owner, gen } => todo!(),
                VMValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
            }
        }
    }

    pub fn downcast_ref<T: AnyValue<'eval>>(&self) -> &T {
        match self {
            VMValue::Moved => todo!(),
            VMValue::Copyable(_) => todo!(),
            VMValue::FullyOwned(_) => todo!(),
            VMValue::EvalPure(value) => value.downcast_ref(),
            VMValue::EvalRef(value) => value.downcast_ref(),
            VMValue::FullyOwnedRef(value) => value.downcast_ref(),
            VMValue::FullyOwnedMut { value, .. } => value.downcast_ref(),
            VMValue::PartiallyOwned(_) => todo!(),
            VMValue::PartiallyOwnedRef(_) => todo!(),
            VMValue::CopyableMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    pub fn downcast_mut<T: AnyValue<'eval>>(&mut self) -> &mut T {
        match self {
            VMValue::Moved => todo!(),
            VMValue::Copyable(_) => todo!(),
            VMValue::FullyOwned(_)
            | VMValue::EvalPure(_)
            | VMValue::EvalRef(_)
            | VMValue::FullyOwnedRef { .. } => {
                panic!()
            }
            VMValue::FullyOwnedMut { ref mut value, .. } => value.downcast_mut(),
            VMValue::PartiallyOwned(_) => todo!(),
            VMValue::PartiallyOwnedRef(_) => todo!(),
            VMValue::CopyableMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    pub fn downcast_mut_full<T: AnyValue<'eval>>(&mut self) -> (&'vm mut T, VMStackIdx, ()) {
        match self {
            VMValue::Moved => todo!(),
            VMValue::Copyable(_) => todo!(),
            VMValue::FullyOwned(_)
            | VMValue::EvalPure(_)
            | VMValue::EvalRef(_)
            | VMValue::FullyOwnedRef { .. } => {
                panic!()
            }
            VMValue::FullyOwnedMut { value, owner, gen } => {
                let ptr: *mut T = value.downcast_mut();
                (unsafe { &mut *ptr }, *owner, *gen)
            }
            VMValue::PartiallyOwned(_) => todo!(),
            VMValue::PartiallyOwnedRef(_) => todo!(),
            VMValue::CopyableMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    pub fn take_copyable(&self) -> CopyableValue {
        match self {
            VMValue::Copyable(value) => *value,
            VMValue::FullyOwnedMut { value, .. } => value.take_copyable(),
            _ => {
                p!(self);
                panic!("")
            }
        }
    }

    pub fn clone_into_stack(&self) -> VMValue<'vm, 'eval> {
        match self {
            VMValue::Moved => todo!(),
            VMValue::Copyable(_) => todo!(),
            VMValue::FullyOwned(_) => todo!(),
            VMValue::EvalPure(value) => VMValue::FullyOwned(value.clone_into_box_dyn().into()),
            VMValue::EvalRef(_) => todo!(),
            VMValue::FullyOwnedRef(value) => VMValue::FullyOwned(value.clone_into_box_dyn().into()),
            VMValue::FullyOwnedMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwned(_) => todo!(),
            VMValue::PartiallyOwnedRef(_) => todo!(),
            VMValue::CopyableMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    pub(crate) fn snapshot(&self) -> StackValueSnapshot<'eval> {
        match self {
            VMValue::Copyable(value) => StackValueSnapshot::Copyable(*value),
            VMValue::FullyOwned(value) => StackValueSnapshot::Owned(value.clone()),
            VMValue::EvalPure(value) => StackValueSnapshot::GlobalPure(value.clone()),
            VMValue::EvalRef(value) => StackValueSnapshot::GlobalRef(*value),
            VMValue::FullyOwnedRef(value) => todo!(),
            VMValue::FullyOwnedMut { value, owner, gen } => {
                p!(value);
                todo!()
            }
            VMValue::Moved => todo!(),
            VMValue::PartiallyOwned(_) => todo!(),
            VMValue::PartiallyOwnedRef(_) => todo!(),
            VMValue::CopyableMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }

    pub fn static_type_id(&self) -> StaticTypeId {
        self.any_ref().static_type_id_dyn()
    }

    pub fn field(
        self,
        field_idx: usize,
        field_access_contract: EagerContract,
    ) -> VMValue<'vm, 'eval> {
        match self {
            VMValue::Moved => todo!(),
            VMValue::Copyable(_) => todo!(),
            VMValue::FullyOwned(boxed_value) => {
                let mut value: VirtualTy = boxed_value.take().unwrap();
                value.take_field(field_idx)
            }
            VMValue::EvalPure(_) => todo!(),
            VMValue::EvalRef(value) => {
                let value: &VirtualTy = value.downcast_ref();
                value.eager_field(field_idx, field_access_contract)
            }
            VMValue::FullyOwnedRef(value) => {
                let value: &VirtualTy = value.downcast_ref();
                value.eager_field(field_idx, field_access_contract)
            }
            VMValue::FullyOwnedMut { value, owner, gen } => {
                let virtual_value: &mut VirtualTy = value.downcast_mut();
                virtual_value.field_mut(field_idx, field_access_contract, owner)
            }
            VMValue::PartiallyOwned(_) => todo!(),
            VMValue::PartiallyOwnedRef(_) => todo!(),
            VMValue::CopyableMut { value, owner, gen } => todo!(),
            VMValue::PartiallyOwnedMut { value, owner, gen } => todo!(),
        }
    }
}
