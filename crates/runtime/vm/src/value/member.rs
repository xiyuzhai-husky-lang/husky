use serde::Serialize;

use crate::*;

#[derive(Debug, Clone)]
pub enum MemberValue<'eval> {
    Copyable(CopyableValue),
    Boxed(OwnedValue<'eval, 'eval>),
    GlobalPure(Arc<dyn AnyValueDyn<'eval> + 'eval>),
    GlobalRef(&'eval (dyn AnyValueDyn<'eval> + 'eval)),
    Moved,
}

impl<'eval> PartialEq for MemberValue<'eval> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Copyable(l0), Self::Copyable(r0)) => l0 == r0,
            (Self::Boxed(l0), Self::Boxed(r0)) => l0 == r0,
            (Self::GlobalPure(l0), Self::GlobalPure(r0)) => todo!(),
            (Self::GlobalRef(l0), Self::GlobalRef(r0)) => todo!(),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl<'eval> Eq for MemberValue<'eval> {}

impl<'vm, 'eval: 'vm> MemberValue<'eval> {
    pub fn into_stack(self) -> VMValue<'vm, 'eval> {
        match self {
            MemberValue::Copyable(value) => VMValue::Copyable(value),
            MemberValue::Boxed(value) => VMValue::FullyOwned(value),
            MemberValue::GlobalPure(value) => VMValue::EvalPure(value),
            MemberValue::GlobalRef(value) => VMValue::EvalRef(value),
            MemberValue::Moved => panic!(),
        }
    }

    pub fn any_ref(&self) -> &(dyn AnyValueDyn<'eval> + 'eval) {
        match self {
            MemberValue::Copyable(_) => todo!(),
            MemberValue::Boxed(ref value) => value.any_ref(),
            MemberValue::GlobalPure(_) => todo!(),
            MemberValue::GlobalRef(_) => todo!(),
            MemberValue::Moved => todo!(),
        }
    }

    pub fn any_ptr(&self) -> *const (dyn AnyValueDyn<'eval> + 'eval) {
        match self {
            MemberValue::Copyable(_) => todo!(),
            MemberValue::Boxed(ref value) => value.any_ref(),
            MemberValue::GlobalPure(_) => todo!(),
            MemberValue::GlobalRef(_) => todo!(),
            MemberValue::Moved => todo!(),
        }
    }

    pub fn stack_ref(&self) -> VMValue<'vm, 'eval> {
        VMValue::FullyOwnedRef(unsafe { &*self.any_ptr() })
    }

    pub fn stack_mut<'a>(&'a mut self, owner: VMStackIdx) -> VMValue<'vm, 'eval> {
        let value_mut: *mut dyn AnyValueDyn<'eval> = match self {
            MemberValue::Copyable(value) => value.any_mut(),
            MemberValue::Boxed(value) => value.any_mut_ptr(),
            MemberValue::GlobalPure(_) => todo!(),
            MemberValue::GlobalRef(_) => todo!(),
            MemberValue::Moved => todo!(),
        };
        VMValue::FullyOwnedMut {
            value: unsafe { &mut *value_mut },
            owner,
            gen: (),
        }
    }

    pub fn share_globally(&self) -> EvalValue<'eval> {
        match self {
            MemberValue::Copyable(value) => EvalValue::Copyable(*value),
            MemberValue::Boxed(_) => todo!(),
            MemberValue::GlobalPure(_) => todo!(),
            MemberValue::GlobalRef(_) => todo!(),
            MemberValue::Moved => todo!(),
        }
    }

    pub fn copy_into_stack(&self) -> VMValue<'vm, 'eval> {
        match self {
            MemberValue::Copyable(value) => VMValue::Copyable(*value),
            MemberValue::Boxed(_) => todo!(),
            MemberValue::GlobalPure(_) => todo!(),
            MemberValue::GlobalRef(_) => todo!(),
            MemberValue::Moved => todo!(),
        }
    }

    pub fn get_json_value(&self) -> serde_json::value::Value {
        match self {
            MemberValue::Copyable(value) => value.get_primitive_json_value(),
            MemberValue::Boxed(value) => value.get_json_value(),
            MemberValue::GlobalPure(value) => value.get_json_value_dyn(),
            MemberValue::GlobalRef(value) => value.get_json_value_dyn(),
            MemberValue::Moved => todo!(),
        }
    }
}
