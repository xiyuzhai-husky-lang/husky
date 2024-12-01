#![feature(trait_upcasting)]
pub mod db;
pub mod eterner;
pub mod memo;
mod pool;
mod vec_array;

pub use dashmap::DashMap;
pub use eterned_macros::{eterned, memo};
use eterner::EternedEntry;

use self::pool::Pool;
use std::collections::HashMap;

#[derive(Debug, Hash)]
pub struct Eterned<T: 'static>(pub &'static EternedEntry<T>);

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
