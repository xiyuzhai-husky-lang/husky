use std::{borrow::Borrow, fmt::Debug, hash::Hash, ops::Deref};

pub trait InternId:
    'static
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

pub struct BasicInternId<T>
where
    T: 'static + ?Sized,
{
    target: &'static T,
}

impl<T: 'static + ?Sized> PartialEq for BasicInternId<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.target as *const T) == (other.target as *const T)
    }
}

impl<T: 'static + ?Sized> Eq for BasicInternId<T> {}

impl<T: 'static + ?Sized> Hash for BasicInternId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.target as *const T).hash(state);
    }
}

impl<T: 'static + ?Sized> Clone for BasicInternId<T> {
    fn clone(&self) -> Self {
        Self {
            target: self.target,
        }
    }
}

impl<T: 'static + ?Sized> Copy for BasicInternId<T> {}

unsafe impl<T: 'static + ?Sized> std::marker::Sync for BasicInternId<T> {}
unsafe impl<T: 'static + ?Sized> std::marker::Send for BasicInternId<T> {}

impl<T: 'static + ?Sized> std::fmt::Debug for BasicInternId<T>
where
    T: 'static + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.target))
    }
}

impl<T: 'static + ?Sized> Deref for BasicInternId<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.target
    }
}

impl<T: 'static + ?Sized> Borrow<T> for BasicInternId<T> {
    fn borrow(&self) -> &T {
        self.target
    }
}

impl<T: 'static + ?Sized> From<&'static T> for BasicInternId<T> {
    fn from(target: &'static T) -> Self {
        Self { target }
    }
}

impl<T: 'static + ?Sized> InternId for BasicInternId<T> {
    type Thing = T;
}
