use crate::{
    eterner::{Eterner, EternerDyn},
    memo::{IsMemo, MemoJarDyn},
    Eterned,
};
use dashmap::DashMap;
use std::cell::Cell;

#[derive(Default)]
pub struct EternerDb {
    eterners: DashMap<std::any::TypeId, EternerDyn>,
    memo_jars: DashMap<std::any::TypeId, MemoJarDyn>,
}

impl EternerDb {
    pub fn etern<T>(&self, t: T) -> Eterned<T>
    where
        T: Clone + Eq + std::hash::Hash + Send + Sync + 'static,
    {
        self.eterner().etern(t)
    }

    pub fn etern_ref<T, Q>(&self, q: &Q) -> Eterned<T>
    where
        T: std::borrow::Borrow<Q> + for<'a> From<&'a Q>,
        T: Clone + Eq + std::hash::Hash + Send + Sync + 'static,
        Q: Eq + std::hash::Hash + ?Sized,
    {
        self.eterner().etern_ref(q)
    }

    /// this is possible because self.eterners contains pointers to the actual eterners
    pub fn eterner<T: Clone + Eq + std::hash::Hash + Send + Sync + 'static>(&self) -> &Eterner<T> {
        use husky_wild_utils::arb_ref;

        unsafe {
            arb_ref(
                self.eterners
                    .entry(std::any::TypeId::of::<T>())
                    .or_insert_with(|| EternerDyn::new::<T>())
                    .downcast(),
            )
        }
    }

    pub fn memo_jar<M: IsMemo>(&self) -> &M::Jar {
        use husky_wild_utils::arb_ref;

        unsafe {
            arb_ref(
                self.memo_jars
                    .entry(std::any::TypeId::of::<M>())
                    .or_insert_with(|| MemoJarDyn::new::<M>())
                    .downcast(),
            )
        }
    }
}

impl EternerDb {
    pub fn print_debug(&self) {
        println!("eterners len: {:?}", self.eterners.len());
        println!("memo_jars len: {:?}", self.memo_jars.len());
        for entry in self.memo_jars.iter() {
            let (k, v) = entry.pair();
            println!("memo_jar key: {:?}", k);
            println!("memo_jar type name: {:?}", v.type_name());
        }
    }
}
