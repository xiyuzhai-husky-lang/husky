// #![feature(box_syntax)]
mod internal;
mod pool;
mod ptr;

pub use ptr::{DefaultInternedPtr, IsInternPtr};

use std::{borrow::Borrow, fmt::Debug, hash::Hash, marker::PhantomData};
use sync_utils::SafeRwLock;

use internal::InternerInternal;

pub trait Internable {}

pub struct Interner<Ptr: IsInternPtr> {
    internal: SafeRwLock<InternerInternal<Ptr>>,
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

impl<Ptr: IsInternPtr> Interner<Ptr> {
    pub fn empty() -> Self {
        Self {
            internal: SafeRwLock::new(InternerInternal::default()),
        }
    }

    pub fn new_from<I: 'static>(ids: &[I]) -> Self
    where
        Ptr: for<'a> From<&'a I>,
    {
        Self {
            internal: SafeRwLock::new(InternerInternal::new_from(ids)),
        }
    }

    pub fn new(ids: &[Ptr]) -> Self {
        Self {
            internal: SafeRwLock::new(InternerInternal::new(ids)),
        }
    }

    pub fn intern(&self, owned: Ptr::Owned) -> Ptr {
        let result = match self
            .internal
            .read(|internal| internal.ids.get(owned.borrow()).map(|id| *id))
        {
            Some(id) => id,
            None => {
                self.internal
                    .write(|internal| match internal.ids.get(owned.borrow()) {
                        Some(ptr) => *ptr, // this step is lest the value has changed
                        None => {
                            let id = internal.things.len();
                            let owned: &Ptr::Owned = unsafe { &*internal.things.alloc(owned) };
                            let ptr: *const Ptr::T = owned.borrow();
                            let itd: Ptr = Ptr::new_intern_ptr(id, unsafe { &*ptr });
                            internal.ids.insert(unsafe { &*itd.raw() }, itd);
                            itd
                        }
                    })
            }
        };
        return result;
    }

    pub fn intern_borrowed(&self, t: &Ptr::T) -> Ptr {
        let result = match self
            .internal
            .read(|internal| internal.ids.get(t).map(|id| *id))
        {
            Some(ptr) => ptr,
            None => {
                self.internal.write(|internal| match internal.ids.get(t) {
                    Some(ptr) => *ptr, // this step is lest the value has changed
                    None => {
                        let id = internal.things.len();
                        let owned: &Ptr::Owned = unsafe { &*internal.things.alloc(t.into()) };
                        let ptr: *const Ptr::T = owned.borrow();
                        let ptr = Ptr::new_intern_ptr(id, unsafe { &*ptr });
                        internal.ids.insert(unsafe { &*ptr.raw() }, ptr);
                        ptr
                    }
                })
            }
        };
        return result;
    }

    pub fn id_iter(&self) -> impl Iterator<Item = Ptr> {
        self.internal
            .read(|internal| internal.id_iter().collect::<Vec<Ptr>>())
            .into_iter()
    }
}
