mod pool;
mod vec_array;

use std::collections::HashMap;

use self::pool::Pool;
pub use interned_macros::interned;
pub use lazy_static::lazy_static;

pub struct Storage<T: 'static, const N: usize> {
    pool: Pool<T, N>,
    map: HashMap<T, Interned<T>>,
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
    pub fn intern(&mut self, t: T) -> Interned<T> {
        if let Some(interned) = self.map.get(&t) {
            return *interned;
        }
        let ptr = self.pool.alloc(t.clone());
        let interned = Interned(unsafe { &*ptr });
        self.map.insert(t, interned);
        interned
    }
}

#[derive(Debug, PartialOrd, Ord, Hash)]
pub struct Interned<T: 'static>(&'static T);

impl<T: 'static> Clone for Interned<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T: 'static> PartialEq for Interned<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 as *const T == other.0 as *const T
    }
}

impl<T: 'static> Eq for Interned<T> {}

impl<T: 'static> Copy for Interned<T> {}

unsafe impl<T> Send for Interned<T> {}

impl<T: 'static> std::ops::Deref for Interned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
