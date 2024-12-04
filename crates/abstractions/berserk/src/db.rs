use crate::{
    berserker::{Berserker, BerserkerDyn},
    memo::{IsMemo, MemoJarDyn},
    AsStatic, Berserk,
};
use dashmap::DashMap;
use std::cell::Cell;

#[derive(Default)]
pub struct BerserkerDb {
    berserkers: DashMap<std::any::TypeId, BerserkerDyn>,
    memo_jars: DashMap<std::any::TypeId, MemoJarDyn>,
}

impl BerserkerDb {
    pub fn berserk<'db, T>(&'db self, t: T) -> Berserk<'db, T>
    where
        T: AsStatic,
        T::Static: Clone + Eq + std::hash::Hash + Send + Sync + 'static,
    {
        unsafe { std::mem::transmute(self.berserker::<T::Static>().berserk(t.as_static())) }
    }

    pub fn berserk_ref<'db, T, Q>(&'db self, q: &Q) -> Berserk<'db, T>
    where
        T: AsStatic,
        T::Static: Clone + Eq + std::hash::Hash + Send + Sync,
        T: std::borrow::Borrow<Q> + for<'a> From<&'a Q>,
        Q: Eq + std::hash::Hash + ?Sized,
    {
        unsafe { std::mem::transmute(self.berserker::<T::Static>().berserk_ref(q)) }
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
