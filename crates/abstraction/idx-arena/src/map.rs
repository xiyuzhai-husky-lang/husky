use husky_check_utils::should;
use std::marker::PhantomData;

use crate::*;

#[derive(Clone)]
pub struct ArenaMap<T, V> {
    data: Vec<Option<V>>,
    phantom: PhantomData<T>,
}

impl<T, V> PartialEq for ArenaMap<T, V>
where
    V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T, V> Eq for ArenaMap<T, V> where V: Eq {}

impl<T, V> std::fmt::Debug for ArenaMap<T, V>
where
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArenaMap")
            .field("data", &self.data)
            .finish()
    }
}

impl<T, V> ArenaMap<T, V> {
    pub fn new(arena: &Arena<T>) -> Self {
        Self {
            data: arena.data.iter().map(|_| None).collect(),
            phantom: PhantomData,
        }
    }

    pub fn get(&self, idx: ArenaIdx<T>) -> Option<&V> {
        match self.data[idx.raw] {
            Some(ref v) => Some(v),
            None => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (ArenaIdx<T>, &V)> {
        self.data.iter().enumerate().filter_map(|(i, v)| match v {
            Some(ref v) => Some((ArenaIdx::new(i), v)),
            None => None,
        })
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.data.iter().filter_map(|v| v.as_ref())
    }

    pub fn insert_new(&mut self, idx: ArenaIdx<T>, v: V) {
        should!(self.data[idx.raw].is_none());
        self.data[idx.raw] = Some(v)
    }
}

impl<T, V> std::ops::Index<ArenaIdx<T>> for ArenaMap<T, V> {
    type Output = V;

    fn index(&self, index: ArenaIdx<T>) -> &Self::Output {
        self.get(index).unwrap()
    }
}
