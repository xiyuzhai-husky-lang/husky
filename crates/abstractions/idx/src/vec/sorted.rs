use std::cmp::Ordering;

use super::*;

#[derive(Clone)]
pub struct SortedIdxVec<I, T> {
    pub vec: IdxVec<I, T>,
    pub sorted: Vec<I>,
}
impl<I, T> std::ops::Deref for SortedIdxVec<I, T> {
    type Target = IdxVec<I, T>;
    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}
impl<I, T> std::ops::DerefMut for SortedIdxVec<I, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}

impl<I, T> Default for SortedIdxVec<I, T> {
    fn default() -> Self {
        Self {
            vec: Default::default(),
            sorted: Default::default(),
        }
    }
}

impl<I: Idx, T: std::fmt::Debug> std::fmt::Debug for SortedIdxVec<I, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.sorted.iter().map(|&i| &self.vec[i]))
            .finish()
    }
}

impl<I: Idx, T> SortedIdxVec<I, T> {
    pub fn equal_range(&self, p: impl Fn(&T) -> Ordering) -> (usize, usize) {
        let start = self
            .sorted
            .partition_point(|&i| p(&self.vec[i]) == Ordering::Less);
        let mut end = start;
        while let Some(&i) = self.sorted.get(end) {
            if p(&self.vec[i]) == Ordering::Greater {
                break;
            }
            end += 1;
        }
        (start, end)
    }

    pub fn find_index(&self, p: impl Fn(&T) -> Ordering) -> Result<I, usize> {
        match self.equal_range(p) {
            (start, end) if start == end => Err(end),
            (start, _) => Ok(self.sorted[start]),
        }
    }

    pub fn find(&self, p: impl Fn(&T) -> Ordering) -> Option<(I, &T)> {
        let i = self.find_index(p).ok()?;
        Some((i, &self.vec[i]))
    }

    pub fn sort_all(&mut self, f: impl Fn(&T, &T) -> Ordering) {
        self.sorted = (0..self.vec.len()).map(Idx::from_usize).collect();
        self.sorted.sort_by(|&a, &b| f(&self.vec[a], &self.vec[b]));
    }

    /// Assumes `idx` is the sorted index of `t` (as returned by `find_index`)
    pub fn insert_at(&mut self, idx: usize, t: T) -> I {
        let i = self.vec.push(t);
        self.sorted.insert(idx, i);
        i
    }

    pub fn truncate(&mut self, len: usize) {
        if len < self.0.len() {
            self.vec.0.truncate(len);
            self.sorted.retain(|t| Idx::into_usize(*t) < len);
            assert!(self.sorted.len() == len)
        }
    }
}
