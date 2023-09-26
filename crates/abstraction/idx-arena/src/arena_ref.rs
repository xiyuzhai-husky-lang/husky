use crate::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ArenaRef<'a, T> {
    data: &'a [T],
}

impl<'a, T> Clone for ArenaRef<'a, T> {
    fn clone(&self) -> Self {
        Self { data: self.data }
    }
}

impl<'a, T> Copy for ArenaRef<'a, T> {}

impl<T> Arena<T> {
    pub fn arena_ref<'a>(&'a self) -> ArenaRef<'a, T> {
        ArenaRef { data: self.data() }
    }
}

impl<'a, T> ArenaRef<'a, T> {
    #[inline]
    pub fn len(self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn data(self) -> &'a [T] {
        self.data
    }

    #[inline]
    pub fn iter(self) -> impl Iterator<Item = &'a T> + 'a {
        self.data.iter()
    }

    pub fn index_iter(&'a self) -> impl Iterator<Item = ArenaIdx<T>> {
        (0..self.data.len()).map(|i| ArenaIdx::new(i))
    }

    pub fn indexed_iter(&'a self) -> impl Iterator<Item = (ArenaIdx<T>, &'a T)> + 'a {
        self.data
            .iter()
            .enumerate()
            .map(|(i, t)| (ArenaIdx::new(i), t))
    }

    pub fn indexed_copy_iter(&'a self) -> impl Iterator<Item = (ArenaIdx<T>, T)> + 'a
    where
        T: Copy,
    {
        self.data
            .iter()
            .copied()
            .enumerate()
            .map(|(i, t)| (ArenaIdx::new(i), t))
    }

    pub fn indexed_iter_with_start(
        &'a self,
        start: usize,
    ) -> impl Iterator<Item = (ArenaIdx<T>, &'a T)> + 'a {
        self.data[start..]
            .iter()
            .enumerate()
            .map(move |(i, t)| (ArenaIdx::new(start + i), t))
    }

    pub fn find_rev_indexed(&self, f: impl Fn(&T) -> bool) -> Option<(ArenaIdx<T>, &T)> {
        self.data.iter().rev().position(|t| f(t)).map(|raw_rev| {
            let raw = self.data.len() - raw_rev - 1;
            (ArenaIdx::new(raw), &self.data[raw])
        })
    }

    pub fn get(self, idx: ArenaIdx<T>) -> Option<&'a T> {
        self.data.get(idx.index())
    }
}

impl<'a, T> core::ops::Index<ArenaIdx<T>> for ArenaRef<'a, T> {
    type Output = T;

    fn index(&self, idx: ArenaIdx<T>) -> &Self::Output {
        &self.data[idx.index()]
    }
}
