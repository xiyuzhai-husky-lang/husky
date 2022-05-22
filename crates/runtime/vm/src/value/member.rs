use serde::Serialize;

use crate::*;

#[derive(Debug, Clone)]
pub enum MemberValue<'eval> {
    Primitive(CopyableValue),
    Boxed(OwnedValue<'eval>),
    GlobalPure(Arc<dyn AnyValueDyn<'eval>>),
    GlobalRef(&'eval dyn AnyValueDyn<'eval>),
    Moved,
}

impl<'eval> PartialEq for MemberValue<'eval> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Primitive(l0), Self::Primitive(r0)) => l0 == r0,
            (Self::Boxed(l0), Self::Boxed(r0)) => l0 == r0,
            (Self::GlobalPure(l0), Self::GlobalPure(r0)) => todo!(),
            (Self::GlobalRef(l0), Self::GlobalRef(r0)) => todo!(),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl<'eval> Eq for MemberValue<'eval> {}

impl<'stack, 'eval: 'stack> MemberValue<'eval> {
    pub fn into_stack(self) -> StackValue<'stack, 'eval> {
        match self {
            MemberValue::Primitive(value) => StackValue::Copyable(value),
            MemberValue::Boxed(value) => StackValue::Owned(value),
            MemberValue::GlobalPure(value) => StackValue::GlobalPure(value),
            MemberValue::GlobalRef(value) => StackValue::GlobalRef(value),
            MemberValue::Moved => panic!(),
        }
    }

    pub fn any_ref(&self) -> &dyn AnyValueDyn<'eval> {
        match self {
            MemberValue::Primitive(_) => todo!(),
            MemberValue::Boxed(ref value) => value.any_ref(),
            MemberValue::GlobalPure(_) => todo!(),
            MemberValue::GlobalRef(_) => todo!(),
            MemberValue::Moved => todo!(),
        }
    }

    pub fn any_ptr(&self) -> *const dyn AnyValueDyn<'eval> {
        match self {
            MemberValue::Primitive(_) => todo!(),
            MemberValue::Boxed(ref value) => value.any_ref(),
            MemberValue::GlobalPure(_) => todo!(),
            MemberValue::GlobalRef(_) => todo!(),
            MemberValue::Moved => todo!(),
        }
    }

    pub fn stack_ref(&self) -> StackValue<'stack, 'eval> {
        StackValue::LocalRef(unsafe { &*self.any_ptr() })
    }

    pub fn stack_mut(&mut self, owner: StackIdx) -> StackValue<'stack, 'eval> {
        let value_mut: *mut (dyn AnyValueDyn<'eval> + 'eval) = match self {
            MemberValue::Primitive(value) => value.any_mut(),
            MemberValue::Boxed(value) => value.any_mut_ptr(),
            MemberValue::GlobalPure(_) => todo!(),
            MemberValue::GlobalRef(_) => todo!(),
            MemberValue::Moved => todo!(),
        };
        StackValue::RefMut {
            value: unsafe { &mut *value_mut },
            owner,
            gen: (),
        }
    }

    pub fn share_globally(&self) -> EvalValue<'eval> {
        match self {
            MemberValue::Primitive(value) => EvalValue::Copyable(*value),
            MemberValue::Boxed(_) => todo!(),
            MemberValue::GlobalPure(_) => todo!(),
            MemberValue::GlobalRef(_) => todo!(),
            MemberValue::Moved => todo!(),
        }
    }

    pub fn copy_into_stack(&self) -> StackValue<'stack, 'eval> {
        match self {
            MemberValue::Primitive(value) => StackValue::Copyable(*value),
            MemberValue::Boxed(_) => todo!(),
            MemberValue::GlobalPure(_) => todo!(),
            MemberValue::GlobalRef(_) => todo!(),
            MemberValue::Moved => todo!(),
        }
    }

    pub fn to_json_value(&self) -> serde_json::value::Value {
        match self {
            MemberValue::Primitive(value) => serde_json::value::to_value(value).unwrap(),
            MemberValue::Boxed(value) => value.get_json_value(),
            MemberValue::GlobalPure(value) => value.get_json_value_dyn(),
            MemberValue::GlobalRef(value) => value.get_json_value_dyn(),
            MemberValue::Moved => todo!(),
        }
    }
}
