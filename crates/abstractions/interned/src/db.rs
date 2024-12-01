use crate::interner::Interner;
use dashmap::DashMap;
use std::cell::Cell;

#[derive(Default)]
pub struct InternerDb {
    interners: DashMap<std::any::TypeId, Interner>,
    memo_storages: DashMap<std::any::TypeId, Box<dyn std::any::Any + Send + Sync>>,
}

thread_local! {
    static ATTACHED_INTERNER_DB: Cell<Option<&'static InternerDb>> = Cell::new(None);
}

#[track_caller]
pub fn attached_interner_db() -> &'static InternerDb {
    ATTACHED_INTERNER_DB
        .with(|cell| cell.get())
        .expect("attached interner db not initialized")
}

impl InternerDb {
    pub fn with_attached<R>(&self, f: impl FnOnce() -> R) -> R {
        use husky_wild_utils::arb_ref;

        ATTACHED_INTERNER_DB.with(|cell| cell.set(Some(unsafe { arb_ref(self) })));
        let result = f();
        ATTACHED_INTERNER_DB.with(|cell| cell.set(None));
        result
    }
}
