#![feature(step_trait)]
pub mod map;
#[cfg(test)]
mod tests;

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

impl<T, Db: ?Sized> salsa::DebugWithDb<Db> for Arena<T>
where
    T: salsa::DebugWithDb<Db>,
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        f.debug_struct("Arena")
            .field("data", &self.data.debug_with(db, include_all_fields))
            .finish()
    }
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl<T> Arena<T> {
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

    // pub fn intern(&mut self, item: T, eq: impl Fn(&T, &T) -> bool) -> ArenaIdx<T>
    // where
    //     T: Eq,
    // {
    //     if let Some(position) = self.data.iter().position(|item1| eq(item1, &item)) {
    //         return ArenaIdx {
    //             raw: position,
    //             phantom: PhantomData,
    //         };
    //     };
    //     let idx = ArenaIdx::new(self.data.len());
    //     self.data.push(item);
    //     idx
    // }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub unsafe fn set(&mut self, idx: ArenaIdx<T>, new_value: T) {
        self.data[idx.raw] = new_value
    }

    pub fn index_iter<'a>(&'a self) -> impl Iterator<Item = ArenaIdx<T>> {
        (0..self.data.len()).map(|i| ArenaIdx {
            raw: i,
            phantom: PhantomData,
        })
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

    pub fn indexed_iter_with_start<'a>(
        &'a self,
        start: usize,
    ) -> impl Iterator<Item = (ArenaIdx<T>, &'a T)> + 'a {
        self.data[start..].iter().enumerate().map(move |(i, t)| {
            (
                ArenaIdx {
                    raw: start + i,
                    phantom: PhantomData,
                },
                t,
            )
        })
    }

    pub fn find_rev_indexed(&self, f: impl Fn(&T) -> bool) -> Option<(ArenaIdx<T>, &T)> {
        self.data.iter().rev().position(|t| f(t)).map(|raw_rev| {
            let raw = self.data.len() - raw_rev - 1;
            (
                ArenaIdx {
                    raw,
                    phantom: PhantomData,
                },
                &self.data[raw],
            )
        })
    }
}

pub fn len<T>(range: &ArenaIdxRange<T>) -> usize {
    range.len()
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

impl<T> Default for ArenaIdxRange<T> {
    fn default() -> Self {
        Self {
            start: ArenaIdx {
                raw: 0,
                phantom: PhantomData,
            },
            end: ArenaIdx {
                raw: 0,
                phantom: PhantomData,
            },
        }
    }
}

impl<T> ArenaIdxRange<T> {
    pub fn to_range(&self) -> std::ops::Range<ArenaIdx<T>> {
        self.start..self.end
    }

    pub fn len(&self) -> usize {
        self.end.raw - self.start.raw
    }

    pub fn start(&self) -> ArenaIdx<T> {
        self.start
    }

    pub fn end(&self) -> ArenaIdx<T> {
        self.end
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

pub struct ArenaIdx<T> {
    raw: usize,
    phantom: PhantomData<T>,
}

pub struct OptionArenaIdx<T> {
    raw: usize,
    phantom: PhantomData<T>,
}

impl<T> OptionArenaIdx<T> {
    pub fn into_option(self) -> Option<ArenaIdx<T>> {
        (self.raw != usize::MAX).then_some(ArenaIdx {
            raw: self.raw,
            phantom: PhantomData,
        })
    }
}

impl<T> Default for OptionArenaIdx<T> {
    fn default() -> Self {
        Self {
            raw: usize::MAX,
            phantom: Default::default(),
        }
    }
}

impl<T> From<ArenaIdx<T>> for OptionArenaIdx<T> {
    fn from(value: ArenaIdx<T>) -> Self {
        OptionArenaIdx {
            raw: value.raw,
            phantom: PhantomData,
        }
    }
}

impl<T> Into<Option<ArenaIdx<T>>> for OptionArenaIdx<T> {
    fn into(self) -> Option<ArenaIdx<T>> {
        self.into_option()
    }
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

impl<T> PartialEq for OptionArenaIdx<T> {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl<T> Eq for OptionArenaIdx<T> {}

impl<T> PartialOrd for OptionArenaIdx<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.raw.partial_cmp(&other.raw)
    }
}

impl<T> Ord for OptionArenaIdx<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.raw.cmp(&other.raw)
    }
}

impl<T> Step for ArenaIdx<T> {
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

impl<T> Debug for OptionArenaIdx<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let idx: Option<ArenaIdx<T>> = (*self).into();
        idx.fmt(f)
    }
}

impl<T> Clone for OptionArenaIdx<T> {
    fn clone(&self) -> Self {
        Self {
            raw: self.raw.clone(),
            phantom: PhantomData,
        }
    }
}

impl<T> Copy for OptionArenaIdx<T> {}

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
