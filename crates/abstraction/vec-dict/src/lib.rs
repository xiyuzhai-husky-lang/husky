use std::{marker::PhantomData, ops::Deref, sync::Arc};

pub trait HasKey<K>
where
    K: PartialEq + Eq + Copy,
{
    fn key(&self) -> K;
}

impl<K, T> HasKey<K> for (K, T)
where
    K: PartialEq + Eq + Copy,
{
    fn key(&self) -> K {
        self.0
    }
}

impl<K, T> HasKey<K> for Arc<T>
where
    K: PartialEq + Eq + Copy,
    T: HasKey<K>,
{
    fn key(&self) -> K {
        (**self).key()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VecDict<K, V>
where
    K: PartialEq + Eq + Copy,
    V: HasKey<K>,
{
    entries: Vec<V>,
    phantom: PhantomData<K>,
}

pub type VecPairDict<K, V> = VecDict<K, (K, V)>;

pub struct Repeat {
    pub i: usize,
    pub j: usize,
}

impl<K, Entry> VecDict<K, Entry>
where
    K: PartialEq + Eq + Copy,
    Entry: HasKey<K>,
{
    pub fn data(&self) -> &[Entry] {
        &self.entries
    }

    pub fn from_vec(data: Vec<Entry>) -> Result<Self, Repeat> {
        for i in 0..data.len() {
            for j in (i + 1)..data.len() {
                if data[i].key() == data[j].key() {
                    return Err(Repeat { i, j });
                }
            }
        }
        Ok(Self {
            entries: data,
            phantom: PhantomData,
        })
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn get(&self, key: K) -> Option<&Entry> {
        self.entries.iter().find(|entry| entry.key() == key)
    }

    pub fn iget(&self, key: K) -> Option<(usize, &Entry)> {
        self.entries
            .iter()
            .enumerate()
            .find(|(_, entry)| entry.key() == key)
    }

    pub fn has(&self, key: K) -> bool {
        self.entries
            .iter()
            .find(|entry| entry.key() == key)
            .is_some()
    }

    pub fn get_mut(&mut self, key: K) -> Option<&mut Entry> {
        self.entries.iter_mut().find(|entry| entry.key() == key)
    }

    pub fn insert_new(&mut self, value: Entry) {
        if self.has(value.key()) {
            panic!()
        } else {
            self.entries.push(value)
        }
    }

    pub fn insert(&mut self, value: Entry) {
        if self.has(value.key()) {
            ()
        } else {
            self.entries.push(value)
        }
    }
    pub fn insert_from_ref(&mut self, value: &Entry)
    where
        Entry: Clone,
    {
        if self.has(value.key()) {
            ()
        } else {
            self.entries.push(value.clone())
        }
    }

    pub fn position(&self, key: K) -> Option<usize> {
        self.entries.iter().position(|entry| entry.key() == key)
    }

    pub fn extends(&mut self, other: Self) {
        for v in other.entries {
            self.insert_new(v)
        }
    }

    pub fn extends_from_ref(&mut self, other: &Self)
    where
        Entry: Clone,
    {
        for entry in &other.entries {
            self.insert_from_ref(entry)
        }
    }
}

impl<K, V> FromIterator<V> for VecDict<K, V>
where
    K: PartialEq + Eq + Copy,
    V: HasKey<K>,
{
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        let mut map = Self::default();
        for v in iter {
            map.insert_new(v);
        }
        map
    }
}

impl<K, V> Deref for VecDict<K, V>
where
    K: PartialEq + Eq + Copy,
    V: HasKey<K>,
{
    type Target = [V];

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl<K, V> Default for VecDict<K, V>
where
    K: PartialEq + Eq + Copy,
    V: HasKey<K>,
{
    fn default() -> Self {
        Self {
            entries: vec![],
            phantom: PhantomData,
        }
    }
}

impl<K, V> std::ops::Index<K> for VecDict<K, V>
where
    K: PartialEq + Eq + Clone + Copy,
    V: HasKey<K>,
{
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        self.get(index).unwrap()
    }
}
