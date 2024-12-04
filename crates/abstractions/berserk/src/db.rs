use crate::{
    berserker::{Berserker, BerserkerDyn},
    memo::{IsMemo, MemoJarDyn},
    Berserk,
};
use dashmap::DashMap;
use std::cell::Cell;

#[derive(Default)]
pub struct BerserkerDb {
    berserkers: DashMap<std::any::TypeId, BerserkerDyn>,
    memo_jars: DashMap<std::any::TypeId, MemoJarDyn>,
}

impl BerserkerDb {
    pub fn berserk<T>(&self, t: T) -> Berserk<T>
    where
        T: Clone + Eq + std::hash::Hash + Send + Sync + 'static,
    {
        self.berserker().etern(t)
    }

    pub fn berserk_ref<T, Q>(&self, q: &Q) -> Berserk<T>
    where
        T: std::borrow::Borrow<Q> + for<'a> From<&'a Q>,
        T: Clone + Eq + std::hash::Hash + Send + Sync + 'static,
        Q: Eq + std::hash::Hash + ?Sized,
    {
        self.berserker().etern_ref(q)
    }

    /// this is possible because self.eterners contains pointers to the actual eterners
    pub fn berserker<'db, T>(&'db self) -> &'db Berserker<T>
    where
        T: Clone + Eq + std::hash::Hash + Send + Sync + 'static,
    {
        use husky_wild_utils::arb_ref;

        unsafe {
            arb_ref(
                self.berserkers
                    .entry(std::any::TypeId::of::<T>())
                    .or_insert_with(|| BerserkerDyn::new::<T>())
                    .downcast_ref::<T>(),
            )
        }
    }

    pub fn memo_jar<M: IsMemo>(&self) -> &M::Jar {
        use husky_wild_utils::arb_ref;

        unsafe {
            arb_ref(
                self.memo_jars
                    .entry(std::any::TypeId::of::<M::Jar>())
                    .or_insert_with(|| MemoJarDyn::new::<M>())
                    .downcast(),
            )
        }
    }
}
