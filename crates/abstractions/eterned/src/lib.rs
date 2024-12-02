#![feature(trait_upcasting)]
pub mod as_id;
pub mod db;
pub mod eterner;
pub mod memo;
mod pool;
mod vec_array;

use as_id::AsEternedId;
pub use dashmap::DashMap;
use db::EternerDb;
pub use eterned_macros::{eterned, memo};
use eterner::EternedEntry;

use self::pool::Pool;
use std::collections::HashMap;

#[derive(Hash)]
pub struct Eterned<T: 'static>(pub &'static EternedEntry<T>);

impl<T: 'static> std::fmt::Debug for Eterned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Eterned")
            .field(&(self.0 as *const _))
            .finish()
    }
}

impl<T: 'static> Eterned<T> {
    pub fn raw_ptr(self) -> *const EternedEntry<T> {
        self.0
    }
}

impl<T: 'static> PartialOrd for Eterned<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.raw_ptr().partial_cmp(&other.raw_ptr())
    }
}

impl<T: 'static> Ord for Eterned<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.raw_ptr().cmp(&other.raw_ptr())
    }
}

impl<T: 'static> Clone for Eterned<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T: 'static> PartialEq for Eterned<T> {
    fn eq(&self, other: &Self) -> bool {
        self.raw_ptr() == other.raw_ptr()
    }
}

impl<T: 'static> Eq for Eterned<T> {}

impl<T: 'static> Copy for Eterned<T> {}

unsafe impl<T> Send for Eterned<T> {}

impl<T: Clone + Eq + std::hash::Hash + Send + Sync + 'static> AsEternedId for Eterned<T> {
    fn as_id(self) -> u32 {
        self.0.id
    }

    fn from_id(id: u32, db: &EternerDb) -> Self {
        use husky_wild_utils::arb_ref;

        let eterner = db.eterner::<T>();
        eterner.with_pool(|pool| Eterned(unsafe { arb_ref(&pool[id]) }))
    }
}
