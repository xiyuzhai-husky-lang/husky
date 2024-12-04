#![feature(trait_upcasting)]
pub mod as_id;
pub mod berserker;
pub mod db;
pub mod memo;
mod pool;
mod vec_array;

use as_id::AsBerserkId;
pub use berserk_macros::{berserk, memo};
use berserker::BerserkEntry;
pub use dashmap::DashMap;
use db::BerserkerDb;

use self::pool::Pool;
use std::collections::HashMap;

#[derive(Hash)]
pub struct Berserk<'db, T: 'db>(pub &'db BerserkEntry<T>);

impl<'db, T: 'db> std::fmt::Debug for Berserk<'db, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Berserk")
            .field(&(self.0 as *const _))
            .finish()
    }
}

impl<'db, T: 'db> Berserk<'db, T> {
    pub fn raw_ptr(self) -> *const BerserkEntry<T> {
        self.0
    }
}

impl<'db, T: 'db> PartialOrd for Berserk<'db, T>
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

impl<'db, T: 'db> Ord for Berserk<'db, T>
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

impl<'db, T: 'db> Clone for Berserk<'db, T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<'db, T: 'db> PartialEq for Berserk<'db, T> {
    fn eq(&self, other: &Self) -> bool {
        self.raw_ptr() == other.raw_ptr()
    }
}

impl<'db, T: 'db> Eq for Berserk<'db, T> {}

impl<'db, T: 'db> Copy for Berserk<'db, T> {}

unsafe impl<'db, T: 'db> Send for Berserk<'db, T> {}

impl<'db, T: 'db> AsBerserkId<'db> for Berserk<'db, T>
where
    T: Clone + Eq + std::hash::Hash + Send + Sync + 'static,
{
    fn as_id(self) -> u32 {
        self.0.id
    }

    fn from_id(id: u32, db: &BerserkerDb) -> Self {
        use husky_wild_utils::arb_ref;

        let eterner = db.berserker::<T>();
        eterner.with_pool(|pool| Berserk(unsafe { arb_ref(&pool[id]) }))
    }
}
