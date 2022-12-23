use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct VecSet<K> {
    entries: Vec<K>,
}

impl<K> Default for VecSet<K> {
    fn default() -> Self {
        Self {
            entries: Default::default(),
        }
    }
}

impl<K> Deref for VecSet<K>
where
    K: PartialEq + Eq + Copy,
{
    type Target = [K];

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl<K> FromIterator<K> for VecSet<K> {
    fn from_iter<T: IntoIterator<Item = K>>(t: T) -> Self {
        Self {
            entries: t.into_iter().collect(),
        }
    }
}

impl<K> VecSet<K>
where
    K: PartialEq + Eq + std::fmt::Debug,
{
    pub fn has(&self, key: K) -> bool
    where
        K: Copy,
    {
        self.entries.iter().find(|entry| **entry == key).is_some()
    }

    pub fn contains(&self, key: &K) -> bool {
        self.entries.iter().find(|entry| *entry == key).is_some()
    }

    pub fn insert_new(&mut self, new: K) -> Result<(), InsertEntryRepeatError<K>>
    where
        K: Copy,
    {
        if self.has(new) {
            Err(InsertEntryRepeatError {
                old: self
                    .entries
                    .iter()
                    .position(|entry| *entry == new)
                    .unwrap()
                    .into(),
                new,
            })
        } else {
            self.entries.push(new);
            Ok(())
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear()
    }

    pub fn toggle(&mut self, value: K) {
        if let Some(position) = self.entries.iter().position(|entry| *entry == value) {
            self.entries.remove(position);
        } else {
            self.entries.push(value)
        }
    }

    pub fn to_vec(&self) -> Vec<K>
    where
        K: Copy,
    {
        self.entries.clone()
    }

    pub fn insert(&mut self, value: K)
    where
        K: Copy,
    {
        if self.has(value) {
            ()
        } else {
            self.entries.push(value)
        }
    }

    pub fn insert_move(&mut self, value: K) {
        if self.contains(&value) {
            ()
        } else {
            self.entries.push(value)
        }
    }

    pub fn extend(&mut self, other: &Self)
    where
        K: Copy,
    {
        for entry in &other.entries {
            self.insert(*entry)
        }
    }
}
