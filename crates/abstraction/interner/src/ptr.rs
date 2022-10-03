use std::{borrow::Borrow, ffi::c_void, fmt::Debug, hash::Hash, marker::PhantomData, ops::Deref};

pub trait IsInternPtr:
    'static
    + Debug
    + Hash
    + PartialEq
    + Eq
    + Send
    + Sync
    + Copy
    + Deref<Target = Self::T>
    + Borrow<Self::T>
{
    type T: 'static + Hash + Eq + ?Sized;
    type Owned: 'static
        + Hash
        + Eq
        + Send
        + Sync
        + Debug
        + Clone
        + Borrow<Self::T>
        + for<'a> From<&'a Self::T>;

    fn new_intern_ptr(id: usize, target: &'static Self::T) -> Self;
}

pub struct DefaultInternedPtr<T, Q>
where
    T: 'static + ?Sized,
    Q: Sized,
{
    target: &'static T,
    phantom: PhantomData<Q>,
}

impl<T, Q> DefaultInternedPtr<T, Q> {
    pub unsafe fn from_raw(raw: *const c_void) -> DefaultInternedPtr<T, Q> {
        let raw = raw as *const T;
        let target: &'static T = &*raw;
        Self {
            target,
            phantom: PhantomData,
        }
    }
    pub unsafe fn to_raw(self) -> *const c_void {
        self.target as *const T as *const c_void
    }
}

impl<T: 'static + ?Sized, Q> PartialEq for DefaultInternedPtr<T, Q> {
    fn eq(&self, other: &Self) -> bool {
        (self.target as *const T) == (other.target as *const T)
    }
}

impl<T: 'static + ?Sized, Q> Eq for DefaultInternedPtr<T, Q> {}

impl<T: 'static + ?Sized, Q> Hash for DefaultInternedPtr<T, Q> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.target as *const T).hash(state);
    }
}

impl<T: 'static + ?Sized, Q> Clone for DefaultInternedPtr<T, Q> {
    fn clone(&self) -> Self {
        Self {
            target: self.target,
            phantom: PhantomData,
        }
    }
}

impl<T: 'static + ?Sized, Q> Copy for DefaultInternedPtr<T, Q> {}

unsafe impl<T: 'static + ?Sized, Q> std::marker::Sync for DefaultInternedPtr<T, Q> {}
unsafe impl<T: 'static + ?Sized, Q> std::marker::Send for DefaultInternedPtr<T, Q> {}

impl<T: 'static + ?Sized, Q> std::fmt::Debug for DefaultInternedPtr<T, Q>
where
    T: 'static + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.target))
    }
}

impl<T: 'static + ?Sized, Q> Deref for DefaultInternedPtr<T, Q> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.target
    }
}

impl<T: 'static + ?Sized, Q> Borrow<T> for DefaultInternedPtr<T, Q> {
    fn borrow(&self) -> &T {
        self.target
    }
}

impl<T, Q> IsInternPtr for DefaultInternedPtr<T, Q>
where
    T: 'static + Debug + Hash + Eq + ?Sized,
    Q: 'static + Hash + Eq + Send + Sync + Debug + Clone + Borrow<T> + for<'a> From<&'a T>,
{
    type T = T;

    type Owned = Q;

    fn new_intern_ptr(id: usize, target: &'static Self::T) -> Self {
        Self {
            target,
            phantom: PhantomData,
        }
    }
}
