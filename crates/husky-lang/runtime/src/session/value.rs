use common::*;

use crate::*;

pub enum CachedValueStorage<'eval> {
    Owned(Box<dyn Any>),
    Shared(&'eval dyn Any),
    Undefined,
}

impl<'eval> CachedValueStorage<'eval> {
    pub(super) fn pointer(&self) -> Option<*const (dyn Any)> {
        match self {
            CachedValueStorage::Owned(v) => Some(&**v),
            CachedValueStorage::Shared(v) => Some(&**v),
            CachedValueStorage::Undefined => None,
        }
    }

    pub(super) unsafe fn value(&self) -> CachedValue<'eval> {
        match self.pointer() {
            Some(ptr) => CachedValue::Defined(&*ptr),
            None => CachedValue::Undefined,
        }
    }
}

pub enum CachedValue<'eval> {
    Defined(&'eval dyn Any),
    Undefined,
}

impl<'eval> CachedValue<'eval> {
    pub(super) fn defined(&self) -> &'eval dyn Any {
        match self {
            CachedValue::Defined(v) => *v,
            CachedValue::Undefined => panic!(),
        }
    }
}
