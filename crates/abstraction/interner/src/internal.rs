use std::collections::HashMap;

use crate::{pool::Pool, *};

pub struct InternerInternal<Ptr: IsInternPtr> {
    pub(crate) things: Pool<Ptr::Owned, 10000>,
    pub(crate) ids: HashMap<Ptr::Owned, Ptr>,
}
impl<Ptr: IsInternPtr> Default for InternerInternal<Ptr> {
    fn default() -> Self {
        Self {
            things: Default::default(),
            ids: Default::default(),
        }
    }
}

impl<Ptr: IsInternPtr> InternerInternal<Ptr> {
    pub fn id_iter<'a>(&'a self) -> impl Iterator<Item = Ptr> + 'a {
        self.ids.iter().map(|(_, id)| *id)
    }

    pub fn new_from<I: 'static>(ids: &[I]) -> Self
    where
        Ptr: for<'a> From<&'a I>,
    {
        let ids = HashMap::<Ptr::Owned, Ptr>::from_iter(ids.iter().map(|id| {
            let id: Ptr = id.into();
            ((*id).into(), id)
        }));
        let things = Default::default();
        Self { things, ids }
    }

    pub fn new(ids: &[Ptr]) -> Self {
        let ids =
            HashMap::<Ptr::Owned, Ptr>::from_iter(ids.iter().map(|id: &Ptr| ((**id).into(), *id)));
        Self {
            things: Default::default(),
            ids,
        }
    }
}
