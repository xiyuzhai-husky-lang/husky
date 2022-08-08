use std::{borrow::Borrow, fmt::Debug, hash::Hash, ops::Deref};

pub trait Intern:
    'static
    + Debug
    + Hash
    + PartialEq
    + Eq
    + Send
    + Sync
    + Copy
    + From<&'static Self::Thing>
    + Deref<Target = Self::Thing>
    + Borrow<Self::Thing>
{
    type Thing: 'static + ?Sized;
}

pub struct InternedPtr<T>
where
    T: 'static + ?Sized,
{
    target: &'static T,
}

impl<T> InternedPtr<T> {
    pub unsafe fn from_raw(raw: *const ()) -> InternedPtr<T> {
        let raw = raw as *const T;
        let target: &'static T = &*raw;
        Self { target }
    }
    pub unsafe fn to_raw(self) -> *const () {
        self.target as *const T as *const ()
    }
}

impl<T: 'static + ?Sized> PartialEq for InternedPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.target as *const T) == (other.target as *const T)
    }
}

impl<T: 'static + ?Sized> Eq for InternedPtr<T> {}

impl<T: 'static + ?Sized> Hash for InternedPtr<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.target as *const T).hash(state);
    }
}

impl<T: 'static + ?Sized> Clone for InternedPtr<T> {
    fn clone(&self) -> Self {
        Self {
            target: self.target,
        }
    }
}

impl<T: 'static + ?Sized> Copy for InternedPtr<T> {}

unsafe impl<T: 'static + ?Sized> std::marker::Sync for InternedPtr<T> {}
unsafe impl<T: 'static + ?Sized> std::marker::Send for InternedPtr<T> {}

impl<T: 'static + ?Sized> std::fmt::Debug for InternedPtr<T>
where
    T: 'static + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.target))
    }
}

impl<T: 'static + ?Sized> Deref for InternedPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.target
    }
}

impl<T: 'static + ?Sized> Borrow<T> for InternedPtr<T> {
    fn borrow(&self) -> &T {
        self.target
    }
}

impl<T: 'static + ?Sized> From<&'static T> for InternedPtr<T> {
    fn from(target: &'static T) -> Self {
        Self { target }
    }
}

impl<T: 'static + ?Sized> Intern for InternedPtr<T>
where
    T: Debug,
{
    type Thing = T;
}
