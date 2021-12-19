use std::marker::PhantomData;

#[derive(Debug)]
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
}

pub type ArenaRange<T> = core::ops::Range<ArenaIdx<T>>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ArenaIdx<T> {
    raw: usize,
    phantom: PhantomData<T>,
}

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
