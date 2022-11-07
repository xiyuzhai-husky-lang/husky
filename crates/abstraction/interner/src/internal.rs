use std::collections::HashMap;

use crate::{pool::Pool, *};

pub struct InternerInternal<Ptr: Interned> {
    pub(crate) things: Pool<Ptr::Owned, 10000>,
    pub(crate) ids: HashMap<&'static Ptr::T, Ptr>,
}
impl<Ptr: Interned> Default for InternerInternal<Ptr> {
    fn default() -> Self {
        Self {
            things: Default::default(),
            ids: Default::default(),
        }
    }
}

impl<Ptr: Interned> InternerInternal<Ptr> {
    pub fn id_iter<'a>(&'a self) -> impl Iterator<Item = Ptr> + 'a {
        self.ids.iter().map(|(_, id)| *id)
    }

    pub fn new_from<I: 'static>(ids: &[I]) -> Self
    where
        Ptr: for<'a> From<&'a I>,
    {
        let ids = HashMap::<&'static Ptr::T, Ptr>::from_iter(ids.iter().map(
            |id| -> (&'static Ptr::T, Ptr) {
                let id: Ptr = id.into();
                (unsafe { &*id.raw() }, id)
            },
        ));
        let things = Default::default();
        Self { things, ids }
    }

    pub fn new(ids: &[Ptr]) -> Self {
        let ids = HashMap::<&'static Ptr::T, Ptr>::from_iter(
            ids.iter().map(|id: &Ptr| (unsafe { &*id.raw() }, *id)),
        );
        Self {
            things: Default::default(),
            ids,
        }
    }
}
