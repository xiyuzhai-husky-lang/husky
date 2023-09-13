#![feature(step_trait)]
mod arena;
mod arena_idx;
mod arena_ref;
pub mod map;
pub mod ordered_map;
#[cfg(test)]
mod tests;

pub use self::arena::*;
pub use self::arena_idx::*;
pub use self::arena_ref::*;

use std::{
    fmt::Debug,
    iter::Step,
    marker::PhantomData,
    ops::{Add, Sub},
};

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

// impl<T> core::ops::Index<ArenaIdx<T>> for Vec<T> {
//     type Output = T;

//     fn index(&self, idx: ArenaIdx<T>) -> &Self::Output {
//         &self[idx.index()]
//     }
// }
