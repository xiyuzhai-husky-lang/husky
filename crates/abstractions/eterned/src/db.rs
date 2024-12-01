use crate::{
    eterner::{Eterner, EternerDyn},
    memo::IsMemo,
    Eterned,
};
use dashmap::DashMap;
use std::cell::Cell;

#[derive(Default)]
pub struct EternerDb {
    eterners: DashMap<std::any::TypeId, EternerDyn>,
    memo_storages: DashMap<std::any::TypeId, Box<dyn std::any::Any + Send + Sync>>,
}

thread_local! {
    static ATTACHED_INTERNER_DB: Cell<Option<&'static EternerDb>> = Cell::new(None);
}

#[track_caller]
pub fn attached_interner_db() -> &'static EternerDb {
    ATTACHED_INTERNER_DB
        .with(|cell| cell.get())
        .expect("attached interner db not initialized")
}

impl EternerDb {
    pub fn with_attached<R>(&self, f: impl FnOnce() -> R) -> R {
        use husky_wild_utils::arb_ref;

        let old = ATTACHED_INTERNER_DB.with(|cell| cell.replace(Some(unsafe { arb_ref(self) })));
        let result = f();
        ATTACHED_INTERNER_DB.with(|cell| cell.set(old));
        result
    }

    pub fn etern<T>(&self, t: T) -> Eterned<T>
    where
        T: Eq + std::hash::Hash + Send + Sync + 'static,
    {
        self.eterner_with(|eterner| eterner.etern(t))
    }

    pub fn etern_ref<T, Q>(&self, q: &Q) -> Eterned<T>
    where
        T: std::borrow::Borrow<Q> + for<'a> From<&'a Q>,
        T: Eq + std::hash::Hash + Send + Sync + 'static,
        Q: Eq + std::hash::Hash + ?Sized,
    {
        self.eterner_with(|eterner| eterner.etern_ref(q))
    }

    fn eterner_with<T: Eq + std::hash::Hash + Send + Sync + 'static, R>(
        &self,
        f: impl FnOnce(&Eterner<T>) -> R,
    ) -> R {
        f(self
            .eterners
            .entry(std::any::TypeId::of::<T>())
            .or_insert_with(|| EternerDyn::new::<T>())
            .downcast())
    }

    pub fn memo_jar<M: IsMemo>(&self) -> &M::Jar {
        todo!()
    }
}
