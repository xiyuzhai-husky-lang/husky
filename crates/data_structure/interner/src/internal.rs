use std::ops::Deref;

use shared_arena::SharedArena;

use crate::*;

pub struct InternerInternal<T, Owned, Id>
where
    T: Hash + Eq + 'static + ?Sized,
    Id: InternId<Thing = T>,
    Owned: Hash + Eq + Send + Sync + Debug + Clone + Borrow<T> + for<'a> From<&'a T>,
{
    pub(crate) things: SharedArena<Owned>,
    pub(crate) ids: HashMap<Owned, Id>,
}
impl<T, Owned, Id> Default for InternerInternal<T, Owned, Id>
where
    T: Hash + Eq + 'static + ?Sized,
    Id: InternId<Thing = T>,
    Owned: Hash + Eq + Send + Sync + Debug + Clone + Borrow<T> + for<'a> From<&'a T>,
{
    fn default() -> Self {
        Self {
            things: Default::default(),
            ids: Default::default(),
        }
    }
}

impl<T, Owned, Id> InternerInternal<T, Owned, Id>
where
    T: Hash + Eq + 'static + ?Sized,
    Id: InternId<Thing = T>,
    Owned: Hash + Eq + Send + Sync + Debug + Clone + Borrow<T> + for<'a> From<&'a T>,
{
    pub fn id_iter<'a>(&'a self) -> impl Iterator<Item = Id> + 'a {
        self.ids.iter().map(|(_, id)| *id)
    }

    pub fn new_from<I: 'static>(ids: &[I]) -> Self
    where
        Id: for<'a> From<&'a I>,
    {
        let ids = HashMap::<Owned, Id>::from_iter(
            ids.iter()
                .map(|id| id.into())
                .map(|id: Id| ((*id).into(), id)),
        );
        Self {
            things: Default::default(),
            ids,
        }
    }

    pub fn new(ids: &[Id]) -> Self {
        let ids = HashMap::<Owned, Id>::from_iter(ids.iter().map(|id: &Id| ((**id).into(), *id)));
        Self {
            things: Default::default(),
            ids,
        }
    }
}
