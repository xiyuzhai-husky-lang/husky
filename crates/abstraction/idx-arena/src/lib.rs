#![feature(step_trait)]
pub mod map;

use std::{
    fmt::Debug,
    iter::Step,
    marker::PhantomData,
    ops::{Add, Sub},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arena<T> {
    data: Vec<T>,
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl<T> Arena<T> {
    pub fn alloc_batch(&mut self, item: Vec<T>) -> Option<ArenaIdxRange<T>> {
        if item.len() == 0 {
            return None;
        }
        let start = ArenaIdx::new(self.data.len());
        self.data.extend(item);
        let end = ArenaIdx::new(self.data.len());
        Some(start..end)
    }

    pub fn alloc_one(&mut self, item: T) -> ArenaIdx<T> {
        let idx = ArenaIdx::new(self.data.len());
        self.data.push(item);
        idx
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn indexed_iter<'a>(&'a self) -> impl Iterator<Item = (ArenaIdx<T>, &'a T)> + 'a {
        self.data.iter().enumerate().map(|(i, t)| {
            (
                ArenaIdx {
                    raw: i,
                    phantom: PhantomData,
                },
                t,
            )
        })
    }
}

pub fn len<T>(range: &ArenaIdxRange<T>) -> usize {
    range.end.raw - range.start.raw
}

pub type ArenaIdxRange<T> = core::ops::Range<ArenaIdx<T>>;

pub struct ArenaIdx<T> {
    raw: usize,
    phantom: PhantomData<T>,
}

impl<T> std::fmt::Display for ArenaIdx<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

impl<T> ArenaIdx<T> {
    pub fn raw(self) -> usize {
        self.raw
    }
}

impl<T> std::hash::Hash for ArenaIdx<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

impl<T> Add<usize> for ArenaIdx<T> {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self {
            raw: self.raw + rhs,
            phantom: PhantomData,
        }
    }
}

impl<T> Sub<Self> for ArenaIdx<T> {
    type Output = usize;

    fn sub(self, rhs: Self) -> Self::Output {
        self.raw - rhs.raw
    }
}

impl<T> PartialEq for ArenaIdx<T> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl<T> Eq for ArenaIdx<T> {}

impl<T> PartialOrd for ArenaIdx<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.raw.partial_cmp(&other.raw)
    }
}

impl<T> Ord for ArenaIdx<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.raw.cmp(&other.raw)
    }
}

impl<T> Step for ArenaIdx<T>
where
    T: PartialEq + Clone,
{
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if start.raw <= end.raw {
            Some(end.raw - start.raw)
        } else {
            None
        }
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        Some(Self {
            raw: start.raw + count,
            phantom: PhantomData,
        })
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        if start.raw >= count {
            Some(Self {
                raw: start.raw - count,
                phantom: PhantomData,
            })
        } else {
            None
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

impl<T> Debug for ArenaIdx<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.raw.fmt(f)
    }
}

impl<T> std::ops::Sub<usize> for ArenaIdx<T> {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self {
            raw: self.raw - rhs,
            phantom: PhantomData,
        }
    }
}

impl<T> Clone for ArenaIdx<T> {
    fn clone(&self) -> Self {
        Self {
            raw: self.raw.clone(),
            phantom: PhantomData,
        }
    }
}

impl<T> Copy for ArenaIdx<T> {}

impl<T> ArenaIdx<T> {
    pub(crate) fn new(raw: usize) -> Self {
        Self {
            raw,
            phantom: PhantomData,
        }
    }

    pub fn is(self, raw: usize) -> bool {
        self.raw == raw
    }
}

impl<T> core::ops::Index<ArenaIdx<T>> for Arena<T> {
    type Output = T;

    fn index(&self, idx: ArenaIdx<T>) -> &Self::Output {
        &self.data[idx.raw]
    }
}

impl<T> core::ops::Index<ArenaIdx<T>> for Vec<T> {
    type Output = T;

    fn index(&self, idx: ArenaIdx<T>) -> &Self::Output {
        &self[idx.raw]
    }
}

impl<T> core::ops::Index<&ArenaIdx<T>> for Arena<T> {
    type Output = T;

    fn index(&self, idx: &ArenaIdx<T>) -> &Self::Output {
        &self.data[idx.raw]
    }
}

impl<T> core::ops::Index<ArenaIdxRange<T>> for Arena<T> {
    type Output = [T];

    fn index(&self, idx: ArenaIdxRange<T>) -> &Self::Output {
        &self.data[idx.start.raw..idx.end.raw]
    }
}

impl<T> core::ops::Index<&ArenaIdxRange<T>> for Arena<T> {
    type Output = [T];

    fn index(&self, idx: &ArenaIdxRange<T>) -> &Self::Output {
        &self.data[idx.start.raw..idx.end.raw]
    }
}
