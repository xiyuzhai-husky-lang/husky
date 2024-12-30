#![feature(trait_upcasting)]
pub mod as_id;
pub mod attach;
pub mod db;
pub mod eterner;
pub mod memo;

pub use dashmap::DashMap;
pub use eterned_macros::{eterned, memo};

use as_id::AsEternedId;
use db::EternerDb;
use eterner::EternedEntry;
use pool::Pool;
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

impl<T: 'static> PartialOrd for Eterned<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.0.sha512.cmp(&other.0.sha512) {
            std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
            std::cmp::Ordering::Equal => self.0.value.partial_cmp(&other.0.value),
            std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
        }
    }
}

impl<T: 'static> Ord for Eterned<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.0.sha512.cmp(&other.0.sha512) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => self.0.value.cmp(&other.0.value),
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
        }
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
