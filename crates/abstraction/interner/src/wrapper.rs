use std::{borrow::Borrow, fmt::Debug, ops::Deref};

use optional::{Noned, OptEq};

#[derive(PartialEq, Eq, Hash)]
pub struct InternedRefWrapper<T: ?Sized + 'static>(Option<*const T>);

impl<T: ?Sized + 'static> Debug for InternedRefWrapper<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.deref_static().fmt(f)
    }
}

impl<T: ?Sized + 'static> Clone for InternedRefWrapper<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<T: ?Sized + 'static> InternedRefWrapper<T> {
    pub fn deref_static(self) -> &'static T {
        unsafe { &*self.0.unwrap() }
    }
}

impl<T: ?Sized + 'static> Copy for InternedRefWrapper<T> {}
unsafe impl<T: ?Sized + 'static> Send for InternedRefWrapper<T> {}
unsafe impl<T: ?Sized + 'static> Sync for InternedRefWrapper<T> {}

impl<T: ?Sized + 'static> InternedRefWrapper<T> {
    // can only be constructed from &'static T
    pub fn new(t: &'static T) -> Self {
        Self(Some(t))
    }
}

impl<T: ?Sized + 'static> std::ops::Deref for InternedRefWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.deref_static()
    }
}

impl<T: ?Sized + 'static> Borrow<T> for InternedRefWrapper<T> {
    fn borrow(&self) -> &T {
        self.deref()
    }
}

impl<T: ?Sized + 'static> Noned for InternedRefWrapper<T> {
    fn is_none(&self) -> bool {
        self.0.is_none()
    }

    fn get_none() -> Self {
        Self(None)
    }
}

impl<T: ?Sized + 'static> OptEq for InternedRefWrapper<T> {
    fn opt_eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
