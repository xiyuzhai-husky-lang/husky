use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
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
    K: PartialEq + Eq + Copy + std::fmt::Debug,
{
    pub fn has(&self, key: K) -> bool {
        self.entries.iter().find(|entry| **entry == key).is_some()
    }

    pub fn insert_new(&mut self, new: K) -> Result<(), EntryRepeatError<K>> {
        if self.has(new) {
            let old = loop {
                let entry = self.entries.pop().unwrap();
                if entry == new {
                    break entry;
                }
            };
            Err(EntryRepeatError { old, new })
        } else {
            self.entries.push(new);
            Ok(())
        }
    }

    pub fn insert(&mut self, value: K) {
        if self.has(value) {
            ()
        } else {
            self.entries.push(value)
        }
    }

    pub fn extend(&mut self, other: &Self) {
        for entry in &other.entries {
            self.insert(*entry)
        }
    }
}
