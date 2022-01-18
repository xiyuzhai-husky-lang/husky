use common::*;

use crate::*;

pub enum CachedValue<'eval> {
    Owned(Box<dyn Any>),
    Shared(&'eval dyn Any),
    Undefined,
}

impl<'eval> CachedValue<'eval> {
    pub(super) fn pointer(&self) -> Option<*const (dyn Any)> {
        match self {
            CachedValue::Owned(v) => Some(&**v),
            CachedValue::Shared(v) => Some(&**v),
            CachedValue::Undefined => None,
        }
    }

    pub(super) unsafe fn value(&self) -> DurableValue<'eval> {
        match self.pointer() {
            Some(ptr) => DurableValue::Defined(&*ptr),
            None => DurableValue::Undefined,
        }
    }
}

pub enum DurableValue<'eval> {
    Defined(&'eval dyn Any),
    Undefined,
}

impl<'eval> DurableValue<'eval> {
    pub(super) fn defined(&self) -> &'eval dyn Any {
        match self {
            DurableValue::Defined(v) => *v,
            DurableValue::Undefined => panic!(),
        }
    }
}
