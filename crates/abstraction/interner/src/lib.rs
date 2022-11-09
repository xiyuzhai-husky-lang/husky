// #![feature(box_syntax)]
mod basic;
mod internal;
mod pool;
mod ptr;

pub use ptr::DefaultItd;

use std::borrow::Borrow;
use sync_utils::SafeRwLock;

use internal::InternerInternal;

pub trait Internable: Sized + 'static {
    type BorrowedRaw: Eq + std::hash::Hash;
    type Borrowed<'a>: Copy + Eq + std::hash::Hash;
    type Interned: Copy;
    fn borrow<'a>(&'a self) -> Self::Borrowed<'a>;
    fn new_itr() -> Interner<Self>;
    fn try_direct(&self) -> Option<Self::Interned>;
    fn itd_to_borrowed(itd: Self::Interned) -> Self::Borrowed<'static>;
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

    pub fn new(ids: &[T::Interned]) -> Self
    where
        T::Interned: Into<T::Borrowed<'static>>,
    {
        Self {
            internal: SafeRwLock::new(InternerInternal::new(ids)),
        }
    }

    pub fn intern(&self, owned: T) -> T::Interned {
        todo!()
        // if let Some(itd) = Itd::opt_atom_itd(owned.borrow()) {
        //     return itd;
        // }
        // let result = match self
        //     .internal
        //     .read(|internal| internal.ids.get(owned.borrow()).map(|id| *id))
        // {
        //     Some(id) => id,
        //     None => {
        //         self.internal
        //             .write(|internal| match internal.ids.get(owned.borrow()) {
        //                 Some(ptr) => *ptr, // this step is lest the value has changed
        //                 None => {
        //                     let id = internal.things.len();
        //                     let owned: &Itd::Owned = unsafe { &*internal.things.alloc(owned) };
        //                     let ptr: Itd::Raw = Itd::owned_to_raw(owned);
        //                     let itd: Itd = Itd::new_interned(id, unsafe { &*ptr });
        //                     internal.ids.insert(unsafe { &*ptr }, itd);
        //                     itd
        //                 }
        //             })
        //     }
        // };
        // return result;
    }

    pub fn intern_borrowed<'a>(&self, t: T::Borrowed<'a>) -> T::Interned
    where
        T: for<'b> From<T::Borrowed<'b>>,
    {
        todo!()
        // if let Some(itd) = Itd::opt_atom_itd(t) {
        //     return itd;
        // }
        // let result = match self
        //     .internal
        //     .read(|internal| internal.ids.get(&t).map(|id| *id))
        // {
        //     Some(ptr) => ptr,
        //     None => {
        //         self.internal.write(|internal| match internal.ids.get(&t) {
        //             Some(ptr) => *ptr, // this step is lest the value has changed
        //             None => {
        //                 let id = internal.things.len();
        //                 let owned: &Itd::Owned = unsafe { &*internal.things.alloc(t.into()) };
        //                 let ptr: *const Itd::Ref = owned.borrow();
        //                 let ptr = Itd::new_interned(id, unsafe { &*ptr });
        //                 internal.ids.insert(unsafe { &*ptr.raw() }, ptr);
        //                 ptr
        //             }
        //         })
        //     }
        // };
        // return result;
    }

    pub fn id_iter(&self) -> impl Iterator<Item = T::Interned> {
        self.internal
            .read(|internal| internal.id_iter().collect::<Vec<T::Interned>>())
            .into_iter()
    }
}
