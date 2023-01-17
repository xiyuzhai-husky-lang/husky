use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct VecSet<K> {
    data: Vec<K>,
}

impl<K> Default for VecSet<K> {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl<K> Deref for VecSet<K>
where
    K: PartialEq + Eq + Copy,
{
    type Target = [K];

    fn deref(&self) -> &Self::Target {
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

impl<K> VecSet<K> {
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
