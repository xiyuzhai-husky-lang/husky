use bimap::BiMap;
use std::{borrow::Borrow, fmt::Debug, hash::Hash, marker::PhantomData};
use stdx::sync::ARwLock;

use internal::InternerInternal;

#[derive(Clone)]
pub struct Interner<T, Id = InternId<T>>
where
    T: Hash + Eq + Send + Sync + Clone,
    Id: Hash + Eq + Send + Sync + Copy + From<u32> + Debug,
{
    internal: ARwLock<InternerInternal<T, Id>>,
}

pub struct IdIter<Id>
where
    Id: From<u32>,
{
    next: u32,
    end: u32,
    phantom: PhantomData<Id>,
}

impl<Id> IdIter<Id>
where
    Id: From<u32>,
{
    pub fn new(start: u32, end: u32) -> Self {
        Self {
            next: start,
            end,
            phantom: PhantomData,
        }
    }
}

impl<Id> Iterator for IdIter<Id>
where
    Id: From<u32>,
{
    type Item = Id;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.end {
            let id = Id::from(self.next);
            self.next += 1;
            Some(id)
        } else {
            None
        }
    }
}

impl<T, Id> Interner<T, Id>
where
    T: Hash + Eq + Send + Sync + Clone,
    Id: Hash + Eq + Send + Sync + Copy + From<u32> + Debug,
{
    pub fn new_from<S, V>(entries: Vec<(S, V)>) -> Self
    where
        T: From<S>,
        Id: From<V>,
    {
        Self {
            internal: ARwLock::new(InternerInternal::<T, Id>::new(BiMap::<T, Id>::from_iter(
                entries.into_iter().map(|(s, v)| (s.into(), v.into())),
            ))),
        }
    }

    pub fn empty() -> Self {
        Self {
            internal: ARwLock::new(InternerInternal::<T, Id>::new(BiMap::<T, Id>::default())),
        }
    }

    pub fn id_iter(&self) -> IdIter<Id> {
        self.internal
            .read(|internal| IdIter::<Id>::new(0, internal.next_id_raw))
    }

    pub fn id_by_ref<Q>(&self, raw: &Q) -> Id
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

    pub fn apply<Q, S>(&self, word: Id, f: impl FnOnce(&Q) -> S) -> S
    where
        T: AsRef<Q>,
        Q: ?Sized,
    {
        self.internal
            .read(|internal| f(internal.bimap.get_by_right(&word).expect("yes").as_ref()))
    }

    pub fn clone_thing(&self, word: Id) -> T
    where
        T: Clone,
    {
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
        pub(crate) next_id_raw: u32,
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

#[derive(Hash, PartialEq, Eq, Default)]
pub struct InternId<T> {
    raw: u32,
    phantom: PhantomData<T>,
}
impl<T> std::fmt::Debug for InternId<T> {
    // fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //     f.debug_struct("InternId")
    //         .field("raw", &self.raw)
    //         .field("phantom", &self.phantom)
    //         .finish()
    // }
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.raw))
    }
}
impl<T> Clone for InternId<T> {
    fn clone(&self) -> Self {
        Self {
            raw: self.raw.clone(),
            phantom: PhantomData,
        }
    }
}
impl<T> Copy for InternId<T> {}
impl<T> From<u32> for InternId<T> {
    fn from(raw: u32) -> Self {
        Self {
            raw,
            phantom: PhantomData,
        }
    }
}
