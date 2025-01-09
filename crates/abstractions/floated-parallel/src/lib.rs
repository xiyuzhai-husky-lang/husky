#![feature(trait_upcasting)]
pub mod db;
pub mod floater;
pub mod note;

pub use dashmap::DashMap;
pub use floated_parallel_macros::{floated, note};
pub use husky_wild_utils::arb_ref;

use crate::{db::FloaterDb, floater::FloatedEntry};

#[derive(Hash)]
pub struct Floated<'db, T: 'static>(pub &'db FloatedEntry<T>);

impl<'db, T: 'static> std::fmt::Debug for Floated<'db, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Floated")
            .field(&(self.0 as *const _))
            .finish()
    }
}

impl<'db, T: 'static> Floated<'db, T> {
    pub fn raw_ptr(self) -> *const FloatedEntry<T> {
        self.0
    }
}

impl<'db, T: 'static> PartialOrd for Floated<'db, T>
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

impl<'db, T: 'static> Ord for Floated<'db, T>
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

impl<'db, T: 'static> Clone for Floated<'db, T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<'db, T: 'static> PartialEq for Floated<'db, T> {
    fn eq(&self, other: &Self) -> bool {
        self.raw_ptr() == other.raw_ptr()
    }
}

impl<'db, T: 'static> Eq for Floated<'db, T> {}

impl<'db, T: 'static> Copy for Floated<'db, T> {}

unsafe impl<'db, T> Send for Floated<'db, T> {}
