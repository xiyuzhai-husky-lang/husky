use std::{marker::PhantomData, ops::Deref};

pub trait HasKey<K>: Clone
where
    K: PartialEq + Eq + Copy,
{
    fn key(&self) -> K;
}

impl<K, T> HasKey<K> for (K, T)
where
    K: PartialEq + Eq + Copy,
    T: Clone,
{
    fn key(&self) -> K {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VecDict<K, V>
where
    K: PartialEq + Eq + Copy,
    V: HasKey<K>,
{
    data: Vec<V>,
    phantom: PhantomData<K>,
}

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
        &self.data
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
            data,
            phantom: PhantomData,
        })
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, key: K) -> Option<&Entry> {
        self.data.iter().find(|entry| entry.key() == key)
    }

    pub fn has(&self, key: K) -> bool {
        self.data.iter().find(|entry| entry.key() == key).is_some()
    }

    pub fn get_mut(&mut self, key: K) -> Option<&mut Entry> {
        self.data.iter_mut().find(|entry| entry.key() == key)
    }

    pub fn insert_new(&mut self, value: Entry) {
        if self.has(value.key()) {
            panic!()
        } else {
            self.data.push(value)
        }
    }

    pub fn insert(&mut self, key: K, value: Entry) {
        if self.has(key) {
            ()
        } else {
            self.data.push(value)
        }
    }

    pub fn position(&self, key: K) -> Option<usize> {
        self.data.iter().position(|entry| entry.key() == key)
    }

    pub fn extends(&mut self, other: Self) {
        for v in other.data {
            self.insert_new(v)
        }
    }

    pub fn extends_from_ref(&mut self, other: &Self) {
        for v in &other.data {
            self.insert_new(v.clone())
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
        &self.data
    }
}

impl<K, V> Default for VecDict<K, V>
where
    K: PartialEq + Eq + Copy,
    V: HasKey<K>,
{
    fn default() -> Self {
        Self {
            data: vec![],
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
