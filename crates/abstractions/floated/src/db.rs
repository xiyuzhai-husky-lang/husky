use crate::{
    floater::{Floater, FloaterDyn},
    note::{IsNote, NoteJarDyn},
    Floated,
};
use dashmap::DashMap;
use std::cell::Cell;

#[derive(Default)]
pub struct FloaterDb {
    floaters: DashMap<std::any::TypeId, FloaterDyn>,
    note_jars: DashMap<std::any::TypeId, NoteJarDyn>,
}

impl FloaterDb {
    pub fn float<T>(&self, t: T) -> Floated<T>
    where
        T: Clone + Eq + std::hash::Hash + Send + Sync + 'static,
    {
        self.floater().float(t)
    }

    pub fn float_ref<T, Q>(&self, q: &Q) -> Floated<T>
    where
        T: std::borrow::Borrow<Q> + for<'a> From<&'a Q>,
        T: Clone + Eq + std::hash::Hash + Send + Sync + 'static,
        Q: Eq + std::hash::Hash + ?Sized,
    {
        self.floater().float_ref(q)
    }

    /// this is possible because self.floaters contains pointers to the actual floaters
    pub fn floater<T: Clone + Eq + std::hash::Hash + Send + Sync + 'static>(&self) -> &Floater<T> {
        use husky_wild_utils::arb_ref;

        unsafe {
            arb_ref(
                self.floaters
                    .entry(std::any::TypeId::of::<T>())
                    .or_insert_with(|| FloaterDyn::new::<T>())
                    .downcast(),
            )
        }
    }

    pub fn note_jar<N: IsNote>(&self) -> &N::Jar {
        use husky_wild_utils::arb_ref;

        unsafe {
            arb_ref(
                self.note_jars
                    .entry(std::any::TypeId::of::<N::Jar>())
                    .or_insert_with(|| NoteJarDyn::new::<N>())
                    .downcast(),
            )
        }
    }
}
