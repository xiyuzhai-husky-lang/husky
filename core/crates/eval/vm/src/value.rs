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
pub use eval::*;
pub use member::*;
pub use owned::*;
pub use ref_::*;
pub use xml::*;

use crate::*;
use print_utils::{msg_once, p, ps};
use std::fmt::Write;
use std::sync::Arc;
use word::CustomIdentifier;

// the primary concerns are safety and stability
// this whole vm thing will be replaced by JIT for fast evaluation purposes
// so we don't need to worry too much about speed here
pub enum TempValue<'temp, 'eval: 'temp> {
    Moved,
    Copyable(CopyableValue),
    OwnedEval(OwnedValue<'eval, 'eval>),
    OwnedTemp(OwnedValue<'temp, 'eval>),
    EvalPure(Arc<dyn AnyValueDyn<'eval> + 'eval>),
    EvalRef(EvalRef<'eval>),
    TempRefEval(&'temp (dyn AnyValueDyn<'eval> + 'eval)),
    TempRefTemp(&'temp (dyn AnyValueDyn<'eval> + 'temp)),
    TempRefMutEval {
        value: &'temp mut (dyn AnyValueDyn<'eval> + 'eval),
        owner: VMStackIdx,
        gen: MutRefGenerator,
    },
    TempRefMutTemp {
        value: &'temp mut (dyn AnyValueDyn<'eval> + 'temp),
        owner: VMStackIdx,
        gen: MutRefGenerator,
    },
}

pub type MutRefGenerator = ();

impl<'temp, 'eval: 'temp> std::fmt::Debug for TempValue<'temp, 'eval> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TempValue::Copyable(arg0) => {
                f.write_str("Primitive ")?;
                arg0.fmt(f)
            }
            TempValue::OwnedEval(arg0) => f.debug_tuple("OwnedEval").field(arg0).finish(),
            TempValue::OwnedTemp(arg0) => f.debug_tuple("OwnedTemp").field(arg0).finish(),
            TempValue::EvalPure(arg0) => f.debug_tuple("EvalPure").field(arg0).finish(),
            TempValue::EvalRef(arg0) => f.debug_tuple("EvalRef").field(arg0).finish(),
            TempValue::TempRefEval(value) => f.debug_tuple("TempRefEval").field(value).finish(),
            TempValue::TempRefTemp(value) => f.debug_tuple("TempRefTemp").field(value).finish(),
            TempValue::TempRefMutEval { value, .. } => f
                .debug_tuple("CopyableMutOrTempRefMutEval")
                .field(value)
                .finish(),
            TempValue::TempRefMutTemp { value, owner, gen } => {
                f.debug_tuple("TempRefMutTemp").field(value).finish()
            }
            TempValue::Moved => f.write_str("Moved"),
        }
    }
}

impl<'temp, 'eval: 'temp> TempValue<'temp, 'eval> {
    pub fn print_short(&self) -> String {
        let mut result = String::new();
        match self {
            TempValue::Moved => result.push_str("Moved"),
            TempValue::Copyable(value) => {
                result.push_str("Primitive ");
                result.push_str(&value.any_ref().print_short())
            }
            TempValue::OwnedEval(value) => {
                result.push_str("Boxed ");
                result.push_str(&value.any_ref().print_short())
            }
            TempValue::OwnedTemp(value) => {
                result.push_str("OwnedTemp ");
                result.push_str(&value.any_ref().print_short());
            }
            TempValue::EvalPure(value) => {
                result.push_str("EvalPure ");
                result.push_str(&value.print_short())
            }
            TempValue::EvalRef(value) => {
                result.push_str("EvalRef ");
                result.push_str(&value.print_short());
            }
            TempValue::TempRefEval(value) => {
                result.push_str("TempRefEval ");
                result.push_str(&value.print_short());
            }
            TempValue::TempRefTemp(value) => {
                result.push_str("TempRefTemp ");
                result.push_str(&value.print_short());
            }
            TempValue::TempRefMutEval { value, owner, gen } => {
                result.push_str("CopyableOrTempMutEval ");
                result.push_str(&value.print_short());
                write!(result, " Owner({:?}) ", owner);
            }
            TempValue::TempRefMutTemp { value, owner, gen } => {
                result.push_str("TempRefMutTemp ");
                result.push_str(&value.print_short());
                write!(result, " Owner({:?}) ", owner);
            }
        }
        result
    }
}

impl<'temp, 'eval: 'temp> From<CopyableValue> for TempValue<'temp, 'eval> {
    fn from(value: CopyableValue) -> Self {
        TempValue::Copyable(value)
    }
}

impl<'temp, 'eval: 'temp> From<&CopyableValue> for TempValue<'temp, 'eval> {
    fn from(value: &CopyableValue) -> Self {
        TempValue::Copyable(*value)
    }
}

impl<'temp, 'eval: 'temp> TempValue<'temp, 'eval> {
    pub fn from_eval(eval_value: EvalValue<'eval>) -> VMRuntimeResult<Self> {
        Ok(match eval_value {
            EvalValue::Copyable(value) => Self::Copyable(value),
            EvalValue::Owned(_) => todo!(),
            EvalValue::EvalPure(value) => TempValue::EvalPure(value),
            EvalValue::EvalRef(value) => Self::EvalRef(value),
            EvalValue::Undefined => todo!(),
        })
    }

    pub fn into_eval(self) -> EvalValue<'eval> {
        match self {
            TempValue::Copyable(copyable_value) => EvalValue::Copyable(copyable_value),
            TempValue::OwnedEval(boxed_value) => EvalValue::Owned(boxed_value),
            TempValue::EvalPure(_) => todo!(),
            TempValue::EvalRef(value) => EvalValue::EvalRef(value),
            TempValue::TempRefEval { .. } | TempValue::TempRefMutEval { .. } | TempValue::Moved => {
                panic!()
            }
            TempValue::OwnedTemp(_) => todo!(),
            TempValue::TempRefTemp(_) => todo!(),
            TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
        }
    }

    pub fn eval(&self) -> EvalValue<'eval> {
        match self {
            TempValue::Copyable(primitive_value) => EvalValue::Copyable(*primitive_value),
            TempValue::OwnedEval(boxed_value) => EvalValue::Owned(boxed_value.clone()),
            TempValue::EvalPure(_) => todo!(),
            TempValue::EvalRef(value) => EvalValue::EvalRef(*value),
            TempValue::TempRefEval(value) => EvalValue::Owned(value.clone_into_box_dyn().into()),
            TempValue::OwnedTemp(_) => todo!(),
            TempValue::TempRefTemp(_) => todo!(),
            TempValue::TempRefMutEval { value, owner, gen } => todo!(),
            TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
            _ => panic!(),
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            TempValue::Copyable(v) => v.to_bool(),
            TempValue::TempRefMutEval { value, owner, gen } => value.take_copyable_dyn().to_bool(),
            _ => panic!(),
        }
    }

    pub fn into_member(&mut self) -> MemberValue<'eval> {
        match self {
            TempValue::Copyable(primitive_value) => MemberValue::Copyable(*primitive_value),
            TempValue::OwnedEval(boxed_value) => match std::mem::replace(self, TempValue::Moved) {
                TempValue::OwnedEval(boxed_value) => MemberValue::Boxed(boxed_value),
                _ => panic!(),
            },
            TempValue::EvalPure(_) => todo!(),
            TempValue::EvalRef(value) => MemberValue::EvalRef(*value),
            TempValue::TempRefEval { .. } | TempValue::TempRefMutEval { .. } | TempValue::Moved => {
                panic!()
            }
            TempValue::OwnedTemp(_) => todo!(),
            TempValue::TempRefTemp(_) => todo!(),
            TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
        }
    }

    pub(crate) unsafe fn bind(&mut self, binding: Binding, stack_idx: VMStackIdx) -> Self {
        match binding {
            Binding::EvalRef => self.bind_eval_ref(),
            Binding::TempRef => self.bind_temp_ref(),
            Binding::TempRefMut => self.bind_ref_mut(stack_idx),
            Binding::Move => self.bind_move(),
            Binding::Copy => self.bind_copy(),
        }
    }

    unsafe fn bind_eval_ref(&self) -> Self {
        match self {
            TempValue::EvalRef(value) => TempValue::EvalRef(*value),
            _ => panic!(),
        }
    }

    unsafe fn bind_temp_ref(&self) -> Self {
        match self {
            TempValue::Moved => panic!(),
            TempValue::Copyable(_) => panic!(),
            TempValue::OwnedEval(value) => {
                let ptr: *const dyn AnyValueDyn = value.any_ptr();
                TempValue::TempRefEval(&*ptr)
            }
            TempValue::OwnedTemp(value) => {
                let ptr: *const dyn AnyValueDyn = value.any_ptr();
                TempValue::TempRefTemp(&*ptr)
            }
            TempValue::EvalPure(value) => {
                let ptr: *const dyn AnyValueDyn = &**value;
                TempValue::TempRefEval(&*ptr)
            }
            TempValue::EvalRef(value) => TempValue::TempRefEval(value.0),
            TempValue::TempRefEval(value) => TempValue::TempRefEval(*value),
            TempValue::TempRefTemp(value) => TempValue::TempRefTemp(*value),
            TempValue::TempRefMutEval { value, owner, gen } => {
                let ptr: *const (dyn AnyValueDyn<'eval> + 'eval) = *value;
                TempValue::TempRefEval(&*ptr)
            }
            TempValue::TempRefMutTemp { value, owner, gen } => {
                let ptr: *const (dyn AnyValueDyn<'eval> + 'temp) = *value;
                TempValue::TempRefTemp(&*ptr)
            }
        }
    }

    fn bind_copy(&self) -> Self {
        match self {
            TempValue::Copyable(value) => TempValue::Copyable(*value),
            TempValue::EvalRef(value) => value.take_copyable_dyn().into(),
            TempValue::TempRefMutEval { value, owner, gen } => value.take_copyable_dyn().into(),
            _ => panic!(),
        }
    }

    unsafe fn bind_ref_mut(&mut self, stack_idx: VMStackIdx) -> TempValue<'temp, 'eval> {
        match self {
            TempValue::Copyable(value) => {
                let ptr: *mut dyn AnyValueDyn<'eval> = value.any_mut();
                TempValue::TempRefMutEval {
                    value: &mut *ptr,
                    owner: stack_idx,
                    gen: (),
                }
            }
            TempValue::OwnedEval(value) => {
                let ptr: *mut dyn AnyValueDyn = &mut *value.any_mut_ptr();
                TempValue::TempRefMutEval {
                    value: &mut *ptr,
                    owner: stack_idx,
                    gen: (),
                }
            }
            TempValue::OwnedTemp(value) => TempValue::TempRefMutTemp {
                value: &mut *value.any_mut_ptr(),
                owner: stack_idx,
                gen: (),
            },
            TempValue::TempRefMutTemp { value, owner, gen } => {
                let ptr: *mut dyn AnyValueDyn = *value;
                TempValue::TempRefMutTemp {
                    value: &mut *ptr,
                    owner: *owner,
                    gen: *gen,
                }
            }
            _ => panic!(),
        }
    }

    unsafe fn pure(&self, stack_idx: VMStackIdx) -> Self {
        match self {
            TempValue::Copyable(value) => TempValue::Copyable(*value),
            TempValue::OwnedEval(value) => TempValue::TempRefEval(&*value.any_ptr()),
            TempValue::EvalPure(value) => TempValue::EvalPure(value.clone()),
            TempValue::EvalRef(value) => TempValue::EvalRef(*value),
            TempValue::TempRefEval(value) => TempValue::TempRefEval(*value),
            TempValue::TempRefMutEval { .. } => todo!(),
            TempValue::Moved => todo!(),
            TempValue::OwnedTemp(_) => todo!(),
            TempValue::TempRefTemp(_) => todo!(),
            TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
        }
    }

    pub(crate) fn bind_move(&mut self) -> Self {
        match self {
            TempValue::Moved => todo!(),
            TempValue::Copyable(value) => TempValue::Copyable(*value),
            TempValue::OwnedEval(_) => std::mem::replace(self, TempValue::Moved),
            TempValue::EvalPure(_) => todo!(),
            TempValue::EvalRef(_) => todo!(),
            TempValue::TempRefEval { .. } => todo!(),
            TempValue::TempRefMutEval { value, owner, gen } => todo!(),
            TempValue::OwnedTemp(_) => todo!(),
            TempValue::TempRefTemp(_) => todo!(),
            TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
        }
    }

    pub(crate) fn bind_return(&mut self) -> Self {
        match self {
            TempValue::Moved => todo!(),
            TempValue::Copyable(value) => Self::Copyable(*value),
            TempValue::OwnedEval(_) => std::mem::replace(self, TempValue::Moved),
            TempValue::EvalPure(_) => todo!(),
            TempValue::EvalRef(_) => todo!(),
            TempValue::TempRefEval { .. } => todo!(),
            TempValue::TempRefMutEval { value, owner, gen } => todo!(),
            TempValue::OwnedTemp(_) => todo!(),
            TempValue::TempRefTemp(_) => todo!(),
            TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
        }
    }

    unsafe fn borrow_mut(&mut self, self_stack_idx: VMStackIdx) -> Self {
        Self::TempRefMutEval {
            value: &mut *self.any_mut_ptr(),
            owner: self.owner(self_stack_idx).unwrap(),
            gen: (),
        }
    }

    fn owner(&self, self_stack_idx: VMStackIdx) -> Option<VMStackIdx> {
        match self {
            TempValue::Copyable(_) | TempValue::OwnedEval(_) => Some(self_stack_idx),
            TempValue::EvalRef(_) | TempValue::EvalPure(_) => None,
            TempValue::TempRefEval { .. } => todo!(),
            TempValue::TempRefMutEval { owner, .. } => Some(*owner),
            TempValue::Moved => todo!(),
            TempValue::OwnedTemp(_) => todo!(),
            TempValue::TempRefTemp(_) => todo!(),
            TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
        }
    }

    pub fn any_ref(&self) -> &(dyn AnyValueDyn<'eval> + 'eval) {
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
                TempValue::OwnedEval(value) => value.any_ref(),
                TempValue::EvalPure(value) => (&**value),
                TempValue::EvalRef(_) => todo!(),
                TempValue::TempRefEval(value) => *value,
                TempValue::TempRefMutEval { value, .. } => *value,
                TempValue::Moved => todo!(),
                TempValue::OwnedTemp(_) => todo!(),
                TempValue::TempRefTemp(_) => todo!(),
                TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
            }
        }
    }

    pub fn any_temp_ref(&self) -> &(dyn AnyValueDyn<'eval> + 'temp) {
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
                TempValue::OwnedEval(value) => value.any_ref(),
                TempValue::EvalPure(value) => (&**value),
                TempValue::EvalRef(_) => todo!(),
                TempValue::TempRefEval(value) => *value,
                TempValue::TempRefMutEval { value, .. } => *value,
                TempValue::Moved => todo!(),
                TempValue::OwnedTemp(_) => todo!(),
                TempValue::TempRefTemp(value) => *value,
                TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
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
                TempValue::OwnedEval(value) => value.any_mut_ptr(),
                TempValue::TempRefMutEval { value, .. } => *value,
                TempValue::TempRefEval { .. } => {
                    panic!("TempRef cannot be mutated, this is a bug.")
                }
                TempValue::EvalPure(_) => panic!("GlobalPure cannot be mutated, this is a bug."),
                TempValue::EvalRef(_) => panic!("EvalRef cannot be mutated, this is a bug."),
                TempValue::Moved => panic!("Move cannot be mutated, this is a bug."),
                TempValue::OwnedTemp(_) => todo!(),
                TempValue::TempRefTemp(_) => todo!(),
                TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
            }
        }
    }

    pub fn downcast_ref<T: AnyValue<'eval>>(&self) -> &T {
        match self {
            TempValue::Moved => todo!(),
            TempValue::Copyable(_) => todo!(),
            TempValue::OwnedEval(_) => todo!(),
            TempValue::EvalPure(value) => value.downcast_ref(),
            TempValue::EvalRef(value) => value.downcast_ref(),
            TempValue::TempRefEval(value) => value.downcast_ref(),
            TempValue::TempRefMutEval { value, .. } => value.downcast_ref(),
            TempValue::OwnedTemp(_) => todo!(),
            TempValue::TempRefTemp(_) => todo!(),
            TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
        }
    }

    pub fn downcast_eval_ref<T: AnyValue<'eval>>(&self) -> &'eval T {
        match self {
            TempValue::Moved => todo!(),
            TempValue::Copyable(_) => todo!(),
            TempValue::OwnedEval(_) => todo!(),
            TempValue::EvalPure(value) => panic!(),
            TempValue::EvalRef(value) => value.0.downcast_ref(),
            TempValue::TempRefEval(value) => panic!(),
            TempValue::TempRefMutEval { value, .. } => panic!(),
            TempValue::OwnedTemp(_) => todo!(),
            TempValue::TempRefTemp(_) => todo!(),
            TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
        }
    }

    pub fn downcast_mut<T: AnyValue<'eval>>(&mut self) -> &mut T {
        match self {
            TempValue::Moved => todo!(),
            TempValue::Copyable(_) => todo!(),
            TempValue::OwnedEval(_)
            | TempValue::EvalPure(_)
            | TempValue::EvalRef(_)
            | TempValue::TempRefEval { .. } => {
                panic!()
            }
            TempValue::TempRefMutEval { ref mut value, .. } => value.downcast_mut(),
            TempValue::OwnedTemp(_) => todo!(),
            TempValue::TempRefTemp(_) => todo!(),
            TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
        }
    }

    pub fn downcast_mut_full<T: AnyValue<'eval>>(&mut self) -> (&'temp mut T, VMStackIdx, ()) {
        match self {
            TempValue::Moved => todo!(),
            TempValue::Copyable(_) => todo!(),
            TempValue::OwnedEval(_)
            | TempValue::EvalPure(_)
            | TempValue::EvalRef(_)
            | TempValue::TempRefEval { .. } => {
                panic!()
            }
            TempValue::TempRefMutEval { value, owner, gen } => {
                let ptr: *mut T = value.downcast_mut();
                (unsafe { &mut *ptr }, *owner, *gen)
            }
            TempValue::OwnedTemp(_) => todo!(),
            TempValue::TempRefTemp(_) => todo!(),
            TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
        }
    }

    pub fn take_copyable(&self) -> CopyableValue {
        match self {
            TempValue::Copyable(value) => *value,
            TempValue::TempRefMutEval { value, .. } => value.take_copyable_dyn(),
            _ => {
                p!(self);
                panic!("")
            }
        }
    }

    pub fn clone_into_stack(&self) -> TempValue<'temp, 'eval> {
        match self {
            TempValue::Moved => todo!(),
            TempValue::Copyable(_) => todo!(),
            TempValue::OwnedEval(_) => todo!(),
            TempValue::EvalPure(value) => TempValue::OwnedEval(value.clone_into_box_dyn().into()),
            TempValue::EvalRef(_) => todo!(),
            TempValue::TempRefEval(value) => {
                TempValue::OwnedEval(value.clone_into_box_dyn().into())
            }
            TempValue::TempRefMutEval { value, owner, gen } => todo!(),
            TempValue::OwnedTemp(_) => todo!(),
            TempValue::TempRefTemp(_) => todo!(),
            TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
        }
    }

    pub(crate) fn snapshot(&self) -> StackValueSnapshot<'eval> {
        match self {
            TempValue::Copyable(value) => StackValueSnapshot::Copyable(*value),
            TempValue::OwnedEval(value) => StackValueSnapshot::Owned(value.clone()),
            TempValue::EvalPure(value) => StackValueSnapshot::EvalPure(value.clone()),
            TempValue::EvalRef(value) => StackValueSnapshot::EvalRef(*value),
            TempValue::TempRefEval(value) => {
                StackValueSnapshot::FullyOwnedRef(value.clone_into_arc_dyn())
            }
            TempValue::TempRefMutEval { value, owner, gen } => {
                p!(value);
                todo!()
            }
            TempValue::Moved => todo!(),
            TempValue::OwnedTemp(_) => todo!(),
            TempValue::TempRefTemp(_) => todo!(),
            TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
        }
    }

    pub fn static_type_id(&self) -> StaticTypeId {
        self.any_ref().static_type_id_dyn()
    }

    pub fn field(self, field_idx: usize, field_binding: Binding) -> TempValue<'temp, 'eval> {
        msg_once!("ad hoc");
        match self {
            TempValue::OwnedEval(boxed_value) => {
                let mut value: VirtualTy = boxed_value.take().unwrap();
                value.take_field(field_idx)
            }
            TempValue::EvalPure(_) => todo!(),
            TempValue::EvalRef(value) => {
                let value: &VirtualTy = value.downcast_ref();
                value.access_field(field_idx, field_binding)
            }
            TempValue::TempRefEval(value) => {
                let value: &VirtualTy = value.downcast_ref();
                value.access_field(field_idx, field_binding)
            }
            TempValue::TempRefTemp(value) => {
                let value: &VirtualTy = value.downcast_ref();
                value.access_field(field_idx, field_binding)
            }
            TempValue::TempRefMutEval { value, owner, gen } => {
                let virtual_value: &mut VirtualTy = value.downcast_mut();
                msg_once!("need cleaning");
                virtual_value.field_mut(field_idx, field_binding, owner)
            }
            TempValue::OwnedTemp(_) => todo!(),
            TempValue::TempRefMutTemp { value, owner, gen } => todo!(),
            _ => panic!(),
        }
    }
}
