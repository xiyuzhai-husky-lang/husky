use crate::*;

pub struct TrackableMap<K, V>
where
    K: PartialEq + Eq,
{
    entries: Vec<(K, V)>,
    old_len: usize,
}
pub enum TrackableMapChange<K, V> {
    None,
    Append { new_entries: Vec<(K, V)> },
}

impl<K, V> Trackable for TrackableMap<K, V>
where
    K: PartialEq + Eq + Clone,
    V: Clone,
{
    type Change = TrackableMapChange<K, V>;

    fn take_change(&mut self) -> TrackableTakeChangeM<Self> {
        if self.old_len == self.entries.len() {
            return TrackableTakeChangeM::Ok(TrackableMapChange::None);
        }
        let new_entries = self.entries[self.old_len..]
            .iter()
            .map(|entry| entry.clone())
            .collect();
        self.old_len = self.entries.len();
        TrackableTakeChangeM::Ok(TrackableMapChange::Append { new_entries })
    }
}

impl<K, V> TrackableMap<K, V>
where
    K: PartialEq + Eq,
{
    pub fn contains(&mut self, key0: &K) -> bool {
        self.entries.iter().any(|(key, _)| key == key0)
    }

    pub fn get(&self, key0: &K) -> Option<&V> {
        self.entries
            .iter()
            .find_map(|(key, value)| if key == key0 { Some(value) } else { None })
    }

    pub fn insert_new(&mut self, key: K, value: V) -> TrackableMakeChangeM<Self, ()> {
        assert!(!self.contains(&key));
        self.entries.push((key, value));
        TrackableMakeChangeM::Ok {
            cont: (),
            phantom_state: PhantomData,
        }
    }
}

impl<K, V> Default for TrackableMap<K, V>
where
    K: PartialEq + Eq,
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
    K: PartialEq + Eq,
{
    type Output = V;

    fn index(&self, index: &K) -> &Self::Output {
        self.get(&index).unwrap()
    }
}

impl<K, V> std::ops::Deref for TrackableMap<K, V>
where
    K: PartialEq + Eq,
{
    type Target = [(K, V)];

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}
