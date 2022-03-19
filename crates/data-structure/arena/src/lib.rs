use std::{marker::PhantomData, ops::Index};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arena<T> {
    storage: Vec<T>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
        }
    }
    pub fn alloc(&mut self, item: Vec<T>) -> ArenaRange<T> {
        let start = ArenaIdx::new(self.storage.len());
        self.storage.extend(item);
        let end = ArenaIdx::new(self.storage.len());
        start..end
    }

    pub fn alloc_one(&mut self, item: T) -> ArenaIdx<T> {
        let idx = ArenaIdx::new(self.storage.len());
        self.storage.push(item);
        idx
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }
}

pub fn len<T>(range: &ArenaRange<T>) -> usize {
    range.end.raw - range.start.raw
}

pub type ArenaRange<T> = core::ops::Range<ArenaIdx<T>>;

#[derive(PartialEq, Eq)]
pub struct ArenaIdx<T> {
    raw: usize,
    phantom: PhantomData<T>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ArenaMap<K, V> {
    data: Vec<V>,
    phantom: PhantomData<K>,
}

impl<K, V> Default for ArenaMap<K, V> {
    fn default() -> Self {
        Self {
            data: vec![],
            phantom: Default::default(),
        }
    }
}

impl<K, V> Index<ArenaIdx<K>> for ArenaMap<K, V> {
    type Output = V;

    fn index(&self, index: ArenaIdx<K>) -> &Self::Output {
        &self.data[index.raw]
    }
}

impl<T> std::fmt::Debug for ArenaIdx<T> {
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
}

impl<T> core::ops::Index<ArenaIdx<T>> for Arena<T> {
    type Output = T;

    fn index(&self, idx: ArenaIdx<T>) -> &Self::Output {
        &self.storage[idx.raw]
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
        &self.storage[idx.raw]
    }
}

impl<T> core::ops::Index<ArenaRange<T>> for Arena<T> {
    type Output = [T];

    fn index(&self, idx: ArenaRange<T>) -> &Self::Output {
        &self.storage[idx.start.raw..idx.end.raw]
    }
}

impl<T> core::ops::Index<&ArenaRange<T>> for Arena<T> {
    type Output = [T];

    fn index(&self, idx: &ArenaRange<T>) -> &Self::Output {
        &self.storage[idx.start.raw..idx.end.raw]
    }
}
