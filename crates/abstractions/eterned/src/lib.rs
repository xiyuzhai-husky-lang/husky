#![feature(trait_upcasting)]
pub mod db;
pub mod eterner;
pub mod memo;
mod pool;
mod vec_array;

pub use dashmap::DashMap;
pub use eterned_macros::{eterned, memo};

use self::pool::Pool;
use std::collections::HashMap;

pub struct Storage<T: 'static, const N: usize> {
    pool: Pool<T, N>,
    map: HashMap<T, Eterned<T>>,
}

impl<T, const N: usize> Storage<T, N> {
    pub fn len_checked(&self) -> usize {
        assert_eq!(self.pool.len(), self.map.len());
        self.map.len()
    }
}

impl<T, const N: usize> Default for Storage<T, N> {
    fn default() -> Self {
        Self {
            pool: Pool::default(),
            map: HashMap::default(),
        }
    }
}

impl<T, const N: usize> Storage<T, N>
where
    T: Clone + Eq + std::hash::Hash,
{
    pub fn intern(&mut self, t: T) -> Eterned<T> {
        if let Some(interned) = self.map.get(&t) {
            return *interned;
        }
        let ptr = self.pool.alloc(t.clone());
        let interned = Eterned(unsafe { &*ptr });
        self.map.insert(t, interned);
        interned
    }

    pub fn intern_ref<Q: Eq + std::hash::Hash + ?Sized>(&mut self, q: &Q) -> Eterned<T>
    where
        T: std::borrow::Borrow<Q> + for<'a> From<&'a Q>,
    {
        if let Some(interned) = self.map.get(q) {
            return *interned;
        }
        let t: T = q.into();
        let ptr = self.pool.alloc(t.clone());
        let interned = Eterned(unsafe { &*ptr });
        self.map.insert(t, interned);
        interned
    }
}

#[derive(Debug, Hash)]
pub struct Eterned<T: 'static>(pub &'static T);

impl<T: 'static> PartialOrd for Eterned<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.0 as *const T as usize).partial_cmp(&(other.0 as *const T as usize))
    }
}

impl<T: 'static> Ord for Eterned<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.0 as *const T as usize).cmp(&(other.0 as *const T as usize))
    }
}

impl<T: 'static> Clone for Eterned<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T: 'static> PartialEq for Eterned<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 as *const T == other.0 as *const T
    }
}

impl<T: 'static> Eq for Eterned<T> {}

impl<T: 'static> Copy for Eterned<T> {}

unsafe impl<T> Send for Eterned<T> {}

impl<T: 'static> std::ops::Deref for Eterned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
