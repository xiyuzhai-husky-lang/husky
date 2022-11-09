use std::collections::HashMap;

use crate::{pool::Pool, *};

pub struct InternerInternal<T: Internable> {
    pub(crate) things: Pool<T, 10000>,
    pub(crate) itds: HashMap<T::Borrowed<'static>, T::Interned>,
}
impl<T: Internable> Default for InternerInternal<T> {
    fn default() -> Self {
        Self {
            things: Default::default(),
            itds: Default::default(),
        }
    }
}

impl<T: Internable> InternerInternal<T> {
    pub fn id_iter<'a>(&'a self) -> impl Iterator<Item = T::Interned> + 'a {
        self.itds.iter().map(|(_, itd)| *itd)
    }

    pub fn new_from<I: 'static>(ids: &[I]) -> Self
    where
        T::Interned: for<'a> From<&'a I>,
    {
        let ids = HashMap::<T::Borrowed<'static>, T::Interned>::from_iter(ids.iter().map(
            |id| -> (T::Borrowed<'static>, T::Interned) {
                let id: T::Interned = id.into();
                (T::itd_to_borrowed(id), id)
            },
        ));
        let things = Default::default();
        Self { things, itds: ids }
    }

    pub fn new(ids: &[T::Interned]) -> Self {
        let ids = HashMap::<T::Borrowed<'static>, T::Interned>::from_iter(
            ids.iter()
                .map(|id: &T::Interned| (T::itd_to_borrowed(*id), *id)),
        );
        Self {
            things: Default::default(),
            itds: ids,
        }
    }
}
