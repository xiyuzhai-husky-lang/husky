pub mod ext;
pub mod sorted;

use crate::*;
use std::{
    marker::PhantomData,
    ops::{Index, IndexMut, Range},
};

/// A vector indexed by a custom indexing type `I`, usually a newtyped integer.
pub struct IdxVec<I, T>(pub Vec<T>, PhantomData<I>);

impl<I, T: std::fmt::Debug> std::fmt::Debug for IdxVec<I, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<I, T: Clone> Clone for IdxVec<I, T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<I, T: PartialEq> PartialEq for IdxVec<I, T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<I, T: Eq> Eq for IdxVec<I, T> {}

impl<I, T> IdxVec<I, T> {
    /// Construct a new empty [`IdxVec`].
    #[must_use]
    pub const fn new() -> Self {
        Self(vec![], PhantomData)
    }

    /// Construct a new [`IdxVec`] with the specified capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity).into()
    }

    /// Construct a new [`IdxVec`] by calling the specified function.
    #[must_use]
    pub fn from_fn(size: usize, f: impl FnMut() -> T) -> Self {
        Self::from(std::iter::repeat_with(f).take(size).collect::<Vec<_>>())
    }

    /// Construct a new [`IdxVec`] using the default element `size` times.
    #[must_use]
    pub fn from_default(size: usize) -> Self
    where
        T: Default,
    {
        Self::from_fn(size, T::default)
    }

    /// The number of elements in the [`IdxVec`].
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Get a value by index into the vector.
    pub fn get(&self, index: I) -> Option<&T>
    where
        I: Idx,
    {
        self.0.get(I::into_usize(index))
    }

    /// Get a value by index into the vector.
    pub fn get_mut(&mut self, index: I) -> Option<&mut T>
    where
        I: Idx,
    {
        self.0.get_mut(I::into_usize(index))
    }

    /// Returns the value that would be returned by the next call to `push`.
    pub fn peek(&self) -> I
    where
        I: Idx,
    {
        I::from_usize(self.0.len())
    }

    /// Insert a new value at the end of the vector.
    pub fn push(&mut self, val: T) -> I
    where
        I: Idx,
    {
        let id = self.peek();
        self.0.push(val);
        id
    }

    /// Grow the vector until it is long enough that `vec[idx]` will work.
    pub fn extend_to_include(&mut self, idx: I)
    where
        I: Idx,
        T: Default,
    {
        let n = I::into_usize(idx) + 1;
        if self.0.len() < n {
            self.0.resize_with(n, T::default)
        }
    }

    /// Get the element with index `idx`, extending the vector if necessary.
    pub fn get_mut_extending(&mut self, idx: I) -> &mut T
    where
        I: Idx,
        T: Default,
    {
        self.extend_to_include(idx);
        &mut self[idx]
    }

    /// An iterator including the indexes, like `iter().enumerate()`, as `I`.
    pub fn enum_iter(&self) -> impl DoubleEndedIterator<Item = (I, &T)> + Clone
    where
        I: Idx,
    {
        self.0
            .iter()
            .enumerate()
            .map(|(n, val)| (I::from_usize(n), val))
    }

    /// An iterator including the indexes, like `iter_mut().enumerate()`, as `I`.
    pub fn enum_iter_mut(&mut self) -> impl DoubleEndedIterator<Item = (I, &mut T)>
    where
        I: Idx,
    {
        self.0
            .iter_mut()
            .enumerate()
            .map(|(n, val)| (I::from_usize(n), val))
    }

    /// Returns `true` if the vector contains no elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<I, T> From<Vec<T>> for IdxVec<I, T> {
    fn from(vec: Vec<T>) -> Self {
        Self(vec, PhantomData)
    }
}

impl<I, T> std::iter::FromIterator<T> for IdxVec<I, T> {
    fn from_iter<J: IntoIterator<Item = T>>(iter: J) -> Self {
        Vec::from_iter(iter).into()
    }
}

impl<I, T> Default for IdxVec<I, T> {
    fn default() -> Self {
        vec![].into()
    }
}

impl<I: Idx, T> Index<I> for IdxVec<I, T> {
    type Output = T;
    #[track_caller]
    fn index(&self, index: I) -> &Self::Output {
        &self.0[I::into_usize(index)]
    }
}

impl<I: Idx, T> IndexMut<I> for IdxVec<I, T> {
    #[track_caller]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.0[I::into_usize(index)]
    }
}

impl<I: Idx, T> Index<Range<I>> for IdxVec<I, T> {
    type Output = [T];
    #[track_caller]
    fn index(&self, r: Range<I>) -> &Self::Output {
        &self.0[I::into_usize(r.start)..I::into_usize(r.end)]
    }
}
