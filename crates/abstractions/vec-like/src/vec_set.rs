use crate::{error::InsertEntryRepeatError, *};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct VecSet<K> {
    data: Vec<K>,
}

impl<'a, K> IntoIterator for &'a VecSet<K> {
    type Item = &'a K;

    type IntoIter = impl Iterator<Item = &'a K> + 'a;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<K> VecSet<K> {
    pub fn new_one_elem_set(elem: K) -> Self {
        Self { data: vec![elem] }
    }
}

impl<K> Default for VecSet<K> {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl<K> Deref for VecSet<K> {
    type Target = [K];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<K> AsRef<[K]> for VecSet<K> {
    fn as_ref(&self) -> &[K] {
        &self.data
    }
}

impl<K> FromIterator<K> for VecSet<K> {
    fn from_iter<T: IntoIterator<Item = K>>(t: T) -> Self {
        Self {
            data: t.into_iter().collect(),
        }
    }
}

impl<const n: usize, K> From<[K; n]> for VecSet<K> {
    fn from(value: [K; n]) -> Self {
        Self::from_iter(value.into_iter())
    }
}

impl<K> VecSet<K> {
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

    pub fn to_vec(&self) -> Vec<K>
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
        K: PartialEq + Eq,
    {
        if self.contains(&value) {
            ()
        } else {
            self.data.push(value)
        }
    }

    /// no assumptions
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

    /// returns the correct position of an existing element or inserts a new element and returns its position.
    ///
    /// ```
    /// use vec_like::VecSet;
    /// let mut set = VecSet::<i32>::default();
    /// assert_eq!(set.position_or_insert(2), 0);
    /// assert_eq!(set.position_or_insert(4), 1);
    /// assert_eq!(set.position_or_insert(6), 2);
    /// assert_eq!(set.position_or_insert(8), 3);
    /// assert_eq!(set.position_or_insert(4), 1);
    /// assert_eq!(set.position_or_insert(2), 0);
    /// assert_eq!(set.position_or_insert(10), 4);
    /// ```
    pub fn position_or_insert(&mut self, k: K) -> usize
    where
        K: PartialEq + Eq,
    {
        match self.data.iter().position(|k_| k_ == &k) {
            Some(position) => position,
            None => {
                let position = self.data.len();
                self.data.push(k);
                position
            }
        }
    }
}
