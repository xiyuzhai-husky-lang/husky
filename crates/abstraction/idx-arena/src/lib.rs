#![feature(step_trait)]
mod idx;
pub mod map;
pub mod ordered_map;
#[cfg(test)]
mod tests;

pub use self::idx::*;

use std::{
    fmt::Debug,
    iter::Step,
    marker::PhantomData,
    ops::{Add, Sub},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Arena<T> {
    data: Vec<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArenaRef<'a, T> {
    data: &'a [T],
}

impl<T> Arena<T> {
    pub fn to_ref<'a>(&'a self) -> ArenaRef<'a, T> {
        ArenaRef { data: &self.data }
    }
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl<T, Db: ?Sized> salsa::DebugWithDb<Db> for Arena<T>
where
    T: salsa::DebugWithDb<Db>,
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        f.debug_struct("Arena")
            .field("data", &self.data.debug_with(db, level.next()))
            .finish()
    }
}

impl<T> Arena<T> {
    #[inline]
    pub fn alloc_batch(&mut self, items: impl IntoIterator<Item = T>) -> ArenaIdxRange<T> {
        let start = ArenaIdx::new(self.data.len());
        self.data.extend(items);
        let end = ArenaIdx::new(self.data.len());
        ArenaIdxRange { start, end }
    }

    pub fn alloc_one(&mut self, item: T) -> ArenaIdx<T> {
        let idx = ArenaIdx::new(self.data.len());
        self.data.push(item);
        idx
    }

    pub unsafe fn next_idx(&self) -> ArenaIdx<T> {
        ArenaIdx::new(self.data.len())
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn data(&self) -> &[T] {
        &self.data
    }

    #[inline]
    pub fn set(&mut self, idx: ArenaIdx<T>, new_value: T) {
        self.data[idx.index()] = new_value
    }

    #[inline]
    pub fn update(&mut self, idx: ArenaIdx<T>, f: impl FnOnce(&mut T)) {
        f(&mut self.data[idx.index()])
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> + 'a {
        self.data.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> + 'a {
        self.data.iter_mut()
    }

    pub fn iter_mut_with_start<'a>(
        &'a mut self,
        start: usize,
    ) -> impl Iterator<Item = &'a mut T> + 'a {
        self.data[start..].iter_mut()
    }

    pub fn index_iter<'a>(&'a self) -> impl Iterator<Item = ArenaIdx<T>> {
        (0..self.data.len()).map(|i| ArenaIdx::new(i))
    }

    pub fn indexed_iter<'a>(&'a self) -> impl Iterator<Item = (ArenaIdx<T>, &'a T)> + 'a {
        self.data
            .iter()
            .enumerate()
            .map(|(i, t)| (ArenaIdx::new(i), t))
    }

    pub fn indexed_copy_iter<'a>(&'a self) -> impl Iterator<Item = (ArenaIdx<T>, T)> + 'a
    where
        T: Copy,
    {
        self.data
            .iter()
            .copied()
            .enumerate()
            .map(|(i, t)| (ArenaIdx::new(i), t))
    }

    pub fn indexed_iter_with_start<'a>(
        &'a self,
        start: usize,
    ) -> impl Iterator<Item = (ArenaIdx<T>, &'a T)> + 'a {
        self.data[start..]
            .iter()
            .enumerate()
            .map(move |(i, t)| (ArenaIdx::new(start + i), t))
    }

    pub fn indexed_iter_mut_with_start<'a>(
        &'a mut self,
        start: usize,
    ) -> impl Iterator<Item = (ArenaIdx<T>, &'a mut T)> + 'a {
        self.data[start..]
            .iter_mut()
            .enumerate()
            .map(move |(i, t)| (ArenaIdx::new(start + i), t))
    }

    pub fn find_rev_indexed(&self, f: impl Fn(&T) -> bool) -> Option<(ArenaIdx<T>, &T)> {
        self.data.iter().rev().position(|t| f(t)).map(|raw_rev| {
            let raw = self.data.len() - raw_rev - 1;
            (ArenaIdx::new(raw), &self.data[raw])
        })
    }
}

pub struct ArenaIdxRange<T> {
    start: ArenaIdx<T>,
    end: ArenaIdx<T>,
}

impl<T> std::fmt::Debug for ArenaIdxRange<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ArenaIdxRange")
            .field(&(self.start..self.end))
            .finish()
    }
}

impl<T> Clone for ArenaIdxRange<T> {
    fn clone(&self) -> Self {
        Self {
            start: self.start,
            end: self.end,
        }
    }
}

impl<T> Copy for ArenaIdxRange<T> {}

impl<T> PartialEq for ArenaIdxRange<T> {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl<T> Eq for ArenaIdxRange<T> {}

impl<T> std::hash::Hash for ArenaIdxRange<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.start.hash(state);
        self.end.hash(state);
    }
}

impl<T> IntoIterator for ArenaIdxRange<T> {
    type Item = ArenaIdx<T>;

    type IntoIter = <core::ops::Range<ArenaIdx<T>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.to_range().into_iter()
    }
}

impl<T> IntoIterator for &ArenaIdxRange<T> {
    type Item = ArenaIdx<T>;

    type IntoIter = <core::ops::Range<ArenaIdx<T>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.to_range().clone().into_iter()
    }
}

impl<T> ArenaIdxRange<T> {
    pub fn to_range(&self) -> std::ops::Range<ArenaIdx<T>> {
        self.start..self.end
    }

    pub fn len(&self) -> usize {
        self.end.index() - self.start.index()
    }

    pub fn start(&self) -> ArenaIdx<T> {
        self.start
    }

    pub fn end(&self) -> ArenaIdx<T> {
        self.end
    }

    pub fn contains(&self, idx: ArenaIdx<T>) -> bool {
        self.start <= idx && self.end > idx
    }

    pub fn last(&self) -> Option<ArenaIdx<T>> {
        if self.start < self.end {
            Some(self.end - 1)
        } else {
            None
        }
    }

    pub fn new_single(idx: ArenaIdx<T>) -> Self {
        Self {
            start: idx,
            end: idx + 1,
        }
    }
}

impl<T> FromIterator<T> for Arena<T> {
    fn from_iter<Iter: IntoIterator<Item = T>>(iter: Iter) -> Self {
        Self {
            data: iter.into_iter().collect(),
        }
    }
}

impl<T> core::ops::Index<ArenaIdx<T>> for Arena<T> {
    type Output = T;

    fn index(&self, idx: ArenaIdx<T>) -> &Self::Output {
        &self.data[idx.index()]
    }
}

impl<T> core::ops::Index<ArenaIdx<T>> for Vec<T> {
    type Output = T;

    fn index(&self, idx: ArenaIdx<T>) -> &Self::Output {
        &self[idx.index()]
    }
}

impl<T> core::ops::Index<&ArenaIdx<T>> for Arena<T> {
    type Output = T;

    fn index(&self, idx: &ArenaIdx<T>) -> &Self::Output {
        &self.data[idx.index()]
    }
}

impl<T> core::ops::Index<ArenaIdxRange<T>> for Arena<T> {
    type Output = [T];

    fn index(&self, idx: ArenaIdxRange<T>) -> &Self::Output {
        &self.data[idx.start.index()..idx.end.index()]
    }
}

impl<T> core::ops::Index<&ArenaIdxRange<T>> for Arena<T> {
    type Output = [T];

    fn index(&self, idx: &ArenaIdxRange<T>) -> &Self::Output {
        &self.data[idx.start.index()..idx.end.index()]
    }
}
