use bimap::BiMap;
use std::{borrow::Borrow, fmt::Debug, hash::Hash, marker::PhantomData};
use stdx::sync::ARwLock;

use internal::InternerInternal;

#[derive(Default, Clone)]
pub struct Interner<T, Id = BasicId<T>>
where
    T: Hash + Eq + Send + Sync + Clone,
    Id: Hash + Eq + Send + Sync + Copy + From<u32> + Debug,
{
    internal: ARwLock<InternerInternal<T, Id>>,
}

impl<T, Id> Interner<T, Id>
where
    T: Hash + Eq + Send + Sync + Clone,
    Id: Hash + Eq + Send + Sync + Copy + From<u32> + Debug,
{
    pub fn new(map: BiMap<T, Id>) -> Self {
        Self {
            internal: ARwLock::new(InternerInternal::<T, Id>::new(map)),
        }
    }
    pub fn id_ref<Q>(&self, raw: &Q) -> Id
    where
        T: Borrow<Q> + for<'a> From<&'a Q>,
        Q: Eq + Hash + ?Sized,
    {
        let result = match self
            .internal
            .read(|internal| internal.bimap.get_by_left(raw).map(|e| *e))
        {
            Some(id) => id,
            None => {
                self.internal
                    .write(|internal| match internal.bimap.get_by_left(raw) {
                        Some(id) => *id, // this step is necessary to make sure it's atomic
                        None => {
                            let id = internal.alloc_new_word();
                            internal.bimap.insert(raw.into(), id);
                            id
                        }
                    })
            }
        };
        return result;
    }

    pub fn id(&self, raw: T) -> Id {
        let result = match self
            .internal
            .read(|internal| internal.bimap.get_by_left(&raw).map(|e| *e))
        {
            Some(id) => id,
            None => {
                self.internal
                    .write(|internal| match internal.bimap.get_by_left(&raw) {
                        Some(id) => *id, // this step is necessary to make sure it's atomic
                        None => {
                            let id = internal.alloc_new_word();
                            internal.bimap.insert(raw.into(), id);
                            id
                        }
                    })
            }
        };
        return result;
    }

    pub fn thing(&self, word: Id) -> T {
        self.internal
            .read(|internal| internal.bimap.get_by_right(&word).expect("yes").clone())
    }
}
mod internal {
    use crate::*;

    pub struct InternerInternal<T, Id>
    where
        T: Hash + PartialEq + Eq + Send + Sync + Clone,
        Id: Hash + PartialEq + Eq + Send + Sync + Clone + Copy,
    {
        next_id_raw: u32,
        pub(crate) bimap: BiMap<T, Id>,
    }
    impl<T, Id> Default for InternerInternal<T, Id>
    where
        T: Hash + PartialEq + Eq + Send + Sync + Clone,
        Id: Hash + PartialEq + Eq + Send + Sync + Clone + Copy,
    {
        fn default() -> Self {
            Self {
                next_id_raw: Default::default(),
                bimap: Default::default(),
            }
        }
    }

    impl<T, Id> InternerInternal<T, Id>
    where
        T: Hash + PartialEq + Eq + Send + Sync + Clone,
        Id: Hash + PartialEq + Eq + Send + Sync + Clone + Copy,
    {
        pub(crate) fn new(map: BiMap<T, Id>) -> Self {
            Self {
                next_id_raw: 0,
                bimap: map,
            }
        }

        pub(crate) fn alloc_new_word(&mut self) -> Id
        where
            T: Hash + Eq + Send + Sync + Clone,
            Id: Hash + Eq + Send + Sync + Copy + From<u32>,
        {
            let id = Id::from(self.next_id_raw);
            self.next_id_raw += 1;
            id
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Default)]
pub struct BasicId<T> {
    raw: u32,
    phantom: PhantomData<T>,
}
impl<T> Clone for BasicId<T> {
    fn clone(&self) -> Self {
        Self {
            raw: self.raw.clone(),
            phantom: PhantomData,
        }
    }
}
impl<T> Copy for BasicId<T> {}
impl<T> From<u32> for BasicId<T> {
    fn from(raw: u32) -> Self {
        Self {
            raw,
            phantom: PhantomData,
        }
    }
}
