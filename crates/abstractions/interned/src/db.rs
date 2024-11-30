use crate::interner::Interner;
use dashmap::DashMap;
use std::cell::Cell;

#[derive(Default)]
pub struct InternerDb {
    interners: DashMap<std::any::TypeId, Interner>,
    memo_storages: DashMap<std::any::TypeId, Box<dyn std::any::Any + Send + Sync>>,
}

thread_local! {
    static ATTACHED_INTERNED_DB: Cell<Option<&'static InternerDb>> = Cell::new(None);
}

pub fn attach_interned_db() -> &'static InternerDb {
    ATTACHED_INTERNED_DB.with(|cell| cell.get().unwrap())
}
