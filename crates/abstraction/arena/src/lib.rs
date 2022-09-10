#![feature(step_trait)]
pub mod map;

use std::{
    fmt::Debug,
    iter::Step,
    marker::PhantomData,
    ops::{Add, Sub},
};

#[derive(Clone, PartialEq, Eq)]
pub struct Arena<T> {
    data: Vec<T>,
}

impl<T> Debug for Arena<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\nArena:\n")?;
        for (i, v) in self.data.iter().enumerate() {
            f.write_fmt(format_args!("  #{}: {:?}\n", i, &v))?
        }
        Ok(())
    }
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub fn alloc(&mut self, item: Vec<T>) -> ArenaRange<T> {
        let start = ArenaIdx::new(self.data.len());
        self.data.extend(item);
        let end = ArenaIdx::new(self.data.len());
        start..end
    }

    pub fn alloc_one(&mut self, item: T) -> ArenaIdx<T> {
        let idx = ArenaIdx::new(self.data.len());
        self.data.push(item);
        idx
    }

    pub fn len(&self) -> usize {
        self.data.len()
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

impl<T> PartialOrd for ArenaIdx<T>
where
    T: PartialEq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.raw.partial_cmp(&other.raw) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.phantom.partial_cmp(&other.phantom)
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

// #[derive(Debug, PartialEq, Eq, Clone)]
// pub struct ArenaMap<K, V>
// where
//     V: for<'a> From<&'a K>,
// {
//     data: Vec<V>,
//     phantom: PhantomData<K>,
// }

// impl<K, V> Index<ArenaIdx<K>> for ArenaMap<K, V>
// where
//     V: for<'a> From<&'a K>,
// {
//     type Output = V;

//     fn index(&self, idx: ArenaIdx<K>) -> &Self::Output {
//         &self.data[idx.raw]
//     }
// }

// impl<K, V> IndexMut<ArenaIdx<K>> for ArenaMap<K, V>
// where
//     V: for<'a> From<&'a K>,
// {
//     fn index_mut(&mut self, idx: ArenaIdx<K>) -> &mut Self::Output {
//         &mut self.data[idx.raw]
//     }
// }

// impl<K, V> ArenaMap<K, V>
// where
//     V: for<'a> From<&'a K>,
// {
//     pub fn new(arena: &Arena<K>) -> Self {
//         Self {
//             data: arena.storage.iter().map(|k| k.into()).collect(),
//             phantom: Default::default(),
//         }
//     }

//     pub fn get(&self, idx: ArenaIdx<K>) -> &V {
//         &self.data[idx.raw]
//     }

//     pub fn get_mut(&mut self, idx: ArenaIdx<K>) -> &mut V {
//         &mut self.data[idx.raw]
//     }
// }

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

impl<T> core::ops::Index<ArenaRange<T>> for Arena<T> {
    type Output = [T];

    fn index(&self, idx: ArenaRange<T>) -> &Self::Output {
        &self.data[idx.start.raw..idx.end.raw]
    }
}

impl<T> core::ops::Index<&ArenaRange<T>> for Arena<T> {
    type Output = [T];

    fn index(&self, idx: &ArenaRange<T>) -> &Self::Output {
        &self.data[idx.start.raw..idx.end.raw]
    }
}
