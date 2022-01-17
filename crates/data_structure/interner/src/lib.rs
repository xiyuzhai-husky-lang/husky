mod id;
mod internal;
mod pool;

pub use id::{BasicInternId, InternId};

use common::*;

use std::{borrow::Borrow, fmt::Debug, hash::Hash, marker::PhantomData};
use stdx::sync::ARwLock;

use internal::InternerInternal;

pub struct Interner<T, Owned, Id = BasicInternId<T>>
where
    T: Hash + Eq + 'static + ?Sized,
    Id: InternId<Thing = T>,
    Owned: Hash + Eq + Send + Sync + Debug + Clone + Borrow<T> + for<'a> From<&'a T>,
{
    internal: ARwLock<InternerInternal<T, Owned, Id>>,
    phantom: PhantomData<T>,
}

impl<T, Owned, Id> Clone for Interner<T, Owned, Id>
where
    T: Hash + Eq + 'static + ?Sized,
    Id: InternId<Thing = T>,
    Owned: Hash + Eq + Send + Sync + Debug + Clone + Borrow<T> + for<'a> From<&'a T>,
{
    fn clone(&self) -> Self {
        Self {
            internal: self.internal.clone(),
            phantom: PhantomData,
        }
    }
}

impl<T, Owned, Id> Interner<T, Owned, Id>
where
    T: Hash + Eq + 'static + ?Sized,
    Id: InternId<Thing = T>,
    Owned: Hash + Eq + Send + Sync + Debug + Clone + Borrow<T> + for<'a> From<&'a T>,
{
    pub fn empty() -> Self {
        Self {
            internal: ARwLock::new(InternerInternal::default()),
            phantom: PhantomData,
        }
    }

    pub fn new_from<I: 'static>(ids: &[I]) -> Self
    where
        Id: for<'a> From<&'a I>,
    {
        Self {
            internal: ARwLock::new(InternerInternal::new_from(ids)),
            phantom: PhantomData,
        }
    }

    pub fn new(ids: &[Id]) -> Self {
        Self {
            internal: ARwLock::new(InternerInternal::new(ids)),
            phantom: PhantomData,
        }
    }

    pub fn intern(&self, owned: Owned) -> Id
    where
        T: Debug,
    {
        let result = match self
            .internal
            .read(|internal| internal.ids.get(owned.borrow()).map(|id| *id))
        {
            Some(id) => id,
            None => {
                self.internal
                    .write(|internal| match internal.ids.get(owned.borrow()) {
                        Some(id) => *id, // this step is lest the value has changed
                        None => {
                            let owned: &Owned = unsafe { &*internal.things.alloc(owned) };
                            let ptr: *const T = owned.borrow();
                            let id: Id = unsafe { &*ptr }.into();
                            internal.ids.insert(owned.clone(), id);
                            id
                        }
                    })
            }
        };
        return result;
    }

    pub fn intern_ref(&self, t: &T) -> Id {
        let result = match self
            .internal
            .read(|internal| internal.ids.get(t).map(|id| *id))
        {
            Some(id) => id,
            None => {
                self.internal.write(|internal| match internal.ids.get(t) {
                    Some(id) => *id, // this step is lest the value has changed
                    None => {
                        let owned: &Owned = unsafe { &*internal.things.alloc(t.into()) };
                        let ptr: *const T = owned.borrow();
                        let id = unsafe { &*ptr }.into();
                        internal.ids.insert(owned.clone(), id);
                        id
                    }
                })
            }
        };
        return result;
    }

    pub fn id_iter(&self) -> impl Iterator<Item = Id> {
        self.internal
            .read(|internal| internal.id_iter().collect::<Vec<Id>>())
            .into_iter()
    }
}
