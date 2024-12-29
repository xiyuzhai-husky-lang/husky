#![feature(trait_upcasting)]
pub mod db;
pub mod floater;
pub mod note;

use crate::{db::FloaterDb, floater::FloatedEntry};
pub use dashmap::DashMap;
pub use floated_macros::{floated, note};

#[derive(Hash)]
pub struct Floated<T: 'static>(pub &'static FloatedEntry<T>);

impl<T: 'static> std::fmt::Debug for Floated<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Floated")
            .field(&(self.0 as *const _))
            .finish()
    }
}

impl<T: 'static> Floated<T> {
    pub fn raw_ptr(self) -> *const FloatedEntry<T> {
        self.0
    }
}

impl<T: 'static> PartialOrd for Floated<T>
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

impl<T: 'static> Ord for Floated<T>
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

impl<T: 'static> Clone for Floated<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T: 'static> PartialEq for Floated<T> {
    fn eq(&self, other: &Self) -> bool {
        self.raw_ptr() == other.raw_ptr()
    }
}

impl<T: 'static> Eq for Floated<T> {}

impl<T: 'static> Copy for Floated<T> {}

unsafe impl<T> Send for Floated<T> {}
