use crate::*;
use smallvec::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct OrderedSmallVecSet<K, const N: usize>
where
    [K; N]: Array<Item = K>,
{
    data: SmallVec<[K; N]>,
}

impl<K, const N: usize> OrderedSmallVecSet<K, N>
where
    [K; N]: Array<Item = K>,
{
    pub fn new_one_elem_set(elem: K) -> Self {
        Self {
            data: smallvec![elem],
        }
    }
}

impl<K, const N: usize> Default for OrderedSmallVecSet<K, N>
where
    [K; N]: Array<Item = K>,
{
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl<K, const N: usize> Deref for OrderedSmallVecSet<K, N>
where
    K: PartialEq + Eq + Copy,
    [K; N]: Array<Item = K>,
{
    type Target = [K];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<K, const N: usize> AsRef<[K]> for OrderedSmallVecSet<K, N>
where
    [K; N]: Array<Item = K>,
{
    fn as_ref(&self) -> &[K] {
        &self.data
    }
}

impl<K, const N: usize> FromIterator<K> for OrderedSmallVecSet<K, N>
where
    [K; N]: Array<Item = K>,
{
    fn from_iter<T: IntoIterator<Item = K>>(t: T) -> Self {
        Self {
            data: t.into_iter().collect(),
        }
    }
}

impl<K, const N: usize, const n: usize> From<[K; n]> for OrderedSmallVecSet<K, N>
where
    [K; N]: Array<Item = K>,
{
    fn from(value: [K; n]) -> Self {
        Self::from_iter(value.into_iter())
    }
}

impl<K, const N: usize> OrderedSmallVecSet<K, N>
where
    [K; N]: Array<Item = K>,
{
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn has(&self, key: K) -> bool
    where
        K: Copy + PartialEq + Eq,
    {
        self.data.iter().find(|entry| **entry == key).is_some()
    }
    pub fn has_ref(&self, key: &K) -> bool
    where
        K: PartialEq + Eq,
    {
        self.data.iter().find(|entry| *entry == key).is_some()
    }

    pub fn contains(&self, key: &K) -> bool
    where
        K: PartialEq + Eq,
    {
        self.data.iter().find(|entry| *entry == key).is_some()
    }

    pub fn insert_new(&mut self, new: K) -> Result<(), InsertEntryRepeatError<K>>
    where
        K: Copy + PartialEq + Eq,
    {
        if self.has(new) {
            Err(InsertEntryRepeatError {
                old: self
                    .data
                    .iter()
                    .position(|entry| *entry == new)
                    .unwrap()
                    .into(),
                new,
            })
        } else {
            self.data.push(new);
            Ok(())
        }
    }

    pub fn clear(&mut self) {
        self.data.clear()
    }

    pub fn toggle(&mut self, value: K)
    where
        K: PartialEq + Eq,
    {
        if let Some(position) = self.data.iter().position(|entry| *entry == value) {
            self.data.remove(position);
        } else {
            self.data.push(value)
        }
    }

    pub fn to_vec(&self) -> SmallVec<[K; N]>
    where
        K: Copy,
    {
        self.data.clone()
    }

    pub fn insert(&mut self, value: K)
    where
        K: Copy + PartialEq + Eq,
    {
        if self.has(value) {
            ()
        } else {
            self.data.push(value)
        }
    }

    pub fn insert_move(&mut self, value: K)
    where
        K: Copy + PartialEq + Eq,
    {
        if self.contains(&value) {
            ()
        } else {
            self.data.push(value)
        }
    }

    pub fn extend(&mut self, other: &Self)
    where
        K: Copy + PartialEq + Eq,
    {
        for entry in &other.data {
            self.insert(*entry)
        }
    }

    pub fn data(&self) -> &[K] {
        self.data.as_ref()
    }
}
