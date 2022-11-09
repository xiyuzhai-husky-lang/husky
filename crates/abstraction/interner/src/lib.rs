mod internal;
mod pool;
mod wrapper;

pub use wrapper::InternedRefWrapper;

use std::borrow::Borrow;
use sync_utils::SafeRwLock;

use internal::InternerInternal;

pub trait Internable: Sized + 'static {
    type Ref<'a>: Copy + Eq + std::hash::Hash;
    type Interned: Copy;
    fn new_itr() -> Interner<Self>;
    fn try_direct(&self) -> Option<Self::Interned> {
        Self::try_direct_from_ref(self.as_ref())
    }
    fn try_direct_from_ref<'a>(r: Self::Ref<'a>) -> Option<Self::Interned>;
    fn itd_to_borrowed(itd: Self::Interned) -> Self::Ref<'static>;
    unsafe fn cast_to_static_ref<'a>(r: Self::Ref<'a>) -> Self::Ref<'static>;
    fn as_ref<'a>(&'a self) -> Self::Ref<'a>;
    unsafe fn to_borrowed_static(&self) -> Self::Ref<'static> {
        let this: &'static Self = &*unsafe { self as *const _ };
        this.as_ref()
    }
    fn new_itd(&'static self, idx: usize) -> Self::Interned;
}

pub trait InternBorrowedRaw: Copy + std::hash::Hash {}

impl<T> InternBorrowedRaw for *const T where T: ?Sized + Eq + std::hash::Hash {}

pub struct Interner<T: Internable> {
    internal: SafeRwLock<InternerInternal<T>>,
}

impl<T: Internable> Default for Interner<T> {
    fn default() -> Self {
        T::new_itr()
    }
}

// impl<T, Owned, Id> Clone for Interner<T, Owned, Id>
// where
//     T: Hash + Eq + 'static + ?Sized,
//     Id: Intern<Thing = T>,
//     Owned: Hash + Eq + Send + Sync + Debug + Clone + Borrow<T> + for<'a> From<&'a T>,
// {
//     fn clone(&self) -> Self {
//         Self {
//             internal: self.internal.clone(),
//             phantom: PhantomData,
//         }
//     }
// }

impl<T: Internable> Interner<T> {
    pub fn new_empty() -> Self {
        Self {
            internal: SafeRwLock::new(InternerInternal::default()),
        }
    }

    pub fn new_from<I: 'static>(ids: &[I]) -> Self
    where
        T::Interned: for<'a> From<&'a I>,
    {
        Self {
            internal: SafeRwLock::new(InternerInternal::new_from(ids)),
        }
    }

    pub fn new(ids: &[T::Interned]) -> Self {
        Self {
            internal: SafeRwLock::new(InternerInternal::new(ids)),
        }
    }

    pub fn intern(&self, t: T) -> T::Interned {
        if let Some(itd) = t.try_direct() {
            return itd;
        }
        let result = match self.internal.read(|internal| {
            internal
                .itds
                .get(&unsafe { t.to_borrowed_static() })
                .map(|id| *id)
        }) {
            Some(id) => id,
            None => {
                self.internal.write(|internal| {
                    match internal.itds.get(&unsafe { t.to_borrowed_static() }) {
                        Some(ptr) => *ptr, // this step is lest the value has changed
                        None => {
                            let idx = internal.things.len();
                            let t: &'static T = unsafe { &*internal.things.alloc(t) };
                            internal.itds.insert(t.as_ref(), t.new_itd(idx));
                            t.new_itd(idx)
                        }
                    }
                })
            }
        };
        return result;
    }

    pub fn intern_borrowed<'a>(&self, r: T::Ref<'a>) -> T::Interned
    where
        T: for<'b> From<T::Ref<'b>>,
    {
        if let Some(itd) = T::try_direct_from_ref(r) {
            return itd;
        }
        let result = match self.internal.read(|internal| {
            internal
                .itds
                .get(&unsafe { T::cast_to_static_ref(r) })
                .map(|id| *id)
        }) {
            Some(ptr) => ptr,
            None => {
                self.internal.write(|internal| {
                    match internal.itds.get(&unsafe { T::cast_to_static_ref(r) }) {
                        Some(ptr) => *ptr, // this step is lest the value has changed
                        None => {
                            let idx = internal.things.len();
                            let t: &'static T = unsafe { &*internal.things.alloc(r.into()) };
                            internal.itds.insert(t.as_ref(), t.new_itd(idx));
                            t.new_itd(idx)
                        }
                    }
                })
            }
        };
        return result;
    }

    pub fn id_iter(&self) -> impl Iterator<Item = T::Interned> {
        self.internal
            .read(|internal| internal.id_iter().collect::<Vec<T::Interned>>())
            .into_iter()
    }
}
