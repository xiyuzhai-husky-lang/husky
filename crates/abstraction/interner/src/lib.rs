// #![feature(box_syntax)]
mod internal;
mod pool;
mod ptr;

pub use ptr::{DefaultItd, Interned};

use std::borrow::Borrow;
use sync_utils::SafeRwLock;

use internal::InternerInternal;

pub trait Internable {}

pub struct Interner<Ptr: Interned> {
    internal: SafeRwLock<InternerInternal<Ptr>>,
}

impl<Ptr> Default for Interner<Ptr>
where
    Ptr: Interned,
{
    fn default() -> Self {
        Ptr::new_itr()
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

impl<Itd: Interned> Interner<Itd> {
    pub fn new_empty() -> Self {
        Self {
            internal: SafeRwLock::new(InternerInternal::default()),
        }
    }

    pub fn new_from<I: 'static>(ids: &[I]) -> Self
    where
        Itd: for<'a> From<&'a I>,
    {
        Self {
            internal: SafeRwLock::new(InternerInternal::new_from(ids)),
        }
    }

    pub fn new(ids: &[Itd]) -> Self {
        Self {
            internal: SafeRwLock::new(InternerInternal::new(ids)),
        }
    }

    pub fn intern(&self, owned: Itd::Owned) -> Itd {
        if let Some(itd) = Itd::opt_atom_itd(owned.borrow()) {
            return itd;
        }
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
                            let owned: &Itd::Owned = unsafe { &*internal.things.alloc(owned) };
                            let ptr: *const Itd::T = owned.borrow();
                            let itd: Itd = Itd::new_interned(id, unsafe { &*ptr });
                            internal.ids.insert(unsafe { &*ptr }, itd);
                            itd
                        }
                    })
            }
        };
        return result;
    }

    pub fn intern_borrowed(&self, t: &Itd::T) -> Itd
    where
        Itd::Owned: for<'a> From<&'a Itd::T>,
    {
        if let Some(itd) = Itd::opt_atom_itd(&t) {
            return itd;
        }
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
                        let owned: &Itd::Owned = unsafe { &*internal.things.alloc(t.into()) };
                        let ptr: *const Itd::T = owned.borrow();
                        let ptr = Itd::new_interned(id, unsafe { &*ptr });
                        internal.ids.insert(unsafe { &*ptr.raw() }, ptr);
                        ptr
                    }
                })
            }
        };
        return result;
    }

    pub fn id_iter(&self) -> impl Iterator<Item = Itd> {
        self.internal
            .read(|internal| internal.id_iter().collect::<Vec<Itd>>())
            .into_iter()
    }
}
