use crate::*;
use projectable::{Projectable, Projector};

pub struct ProjMap<K, V>
where
    K: PartialEq + Eq,
{
    entries: Vec<(K, V)>,
    old_len: usize,
}

impl<K, V> ProjMap<K, V>
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

    pub fn insert_new(&mut self, key: K, value: V) -> ProjMakeChangeM<Self, ()> {
        assert!(!self.contains(&key));
        self.entries.push((key, value));
        ProjMakeChangeM::Ok {
            cont: (),
            phantom_state: PhantomData,
        }
    }
}

impl<K, V> Default for ProjMap<K, V>
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

impl<K, V> std::ops::Index<&K> for ProjMap<K, V>
where
    K: PartialEq + Eq,
{
    type Output = V;

    fn index(&self, index: &K) -> &Self::Output {
        self.get(&index).unwrap()
    }
}
