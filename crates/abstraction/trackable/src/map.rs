use crate::*;

pub struct TrackableMap<K, V>
where
    K: PartialEq + Eq,
{
    entries: Vec<(K, V)>,
    old_len: usize,
}
pub struct TrackableMapChange;

impl<K, V> Trackable for TrackableMap<K, V>
where
    K: PartialEq + Eq,
{
    type Change = TrackableMapChange;

    fn take_change(&mut self) -> TrackableTakeChangeM<Self> {
        todo!()
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
