use crate::*;
use serde::{Deserialize, Serialize};

pub struct TrackableMap<K, V>
where
    K: PartialEq + Eq + Clone,
    V: Clone,
{
    entries: Vec<(K, V)>,
    old_len: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TrackableMapChange<K, V> {
    None,
    Append { new_entries: Vec<(K, V)> },
}

impl<K, V> TrackableMapChange<K, V>
where
    K: PartialEq + Eq + Clone,
    V: Clone,
{
    pub fn opt_new_entries(self) -> Option<Vec<(K, V)>> {
        match self {
            TrackableMapChange::None => None,
            TrackableMapChange::Append { new_entries } => Some(new_entries),
        }
    }
}

impl<K, V> Trackable for TrackableMap<K, V>
where
    K: PartialEq + Eq + Clone,
    V: Clone,
{
    type Change = TrackableMapChange<K, V>;

    fn take_change(&mut self) -> Self::Change {
        if self.old_len == self.entries.len() {
            return TrackableMapChange::None;
        }
        let new_entries = self.entries[self.old_len..]
            .iter()
            .map(|entry| entry.clone())
            .collect();
        self.old_len = self.entries.len();
        TrackableMapChange::Append { new_entries }
    }
}

impl<K, V> TrackableMap<K, V>
where
    K: PartialEq + Eq + Clone,
    V: Clone,
{
    pub fn contains(&mut self, key0: &K) -> bool {
        self.entries.iter().any(|(key, _)| key == key0)
    }

    pub fn get(&self, key0: &K) -> Option<&V> {
        self.entries
            .iter()
            .find_map(|(key, value)| if key == key0 { Some(value) } else { None })
    }

    pub fn insert_new(&mut self, key: K, value: V) {
        assert!(!self.contains(&key));
        self.entries.push((key, value))
    }
}

impl<K, V> Default for TrackableMap<K, V>
where
    K: PartialEq + Eq + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self {
            entries: Default::default(),
            old_len: Default::default(),
        }
    }
}

impl<K, V> std::ops::Index<&K> for TrackableMap<K, V>
where
    K: PartialEq + Eq + Clone,
    V: Clone,
{
    type Output = V;

    fn index(&self, index: &K) -> &Self::Output {
        self.get(&index).unwrap()
    }
}

impl<K, V> std::ops::Deref for TrackableMap<K, V>
where
    K: PartialEq + Eq + Clone,
    V: Clone,
{
    type Target = [(K, V)];

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}
