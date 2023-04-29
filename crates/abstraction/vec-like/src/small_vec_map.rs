use crate::*;
use smallvec::{smallvec, Array, SmallVec};
use thiserror::Error;

#[derive(PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct SmallVecMap<V, const N: usize>
where
    [V; N]: Array<Item = V>,
{
    data: SmallVec<[V; N]>,
}

impl<E, const N: usize> IntoIterator for SmallVecMap<E, N>
where
    [E; N]: Array<Item = E>,
{
    type Item = E;

    type IntoIter = smallvec::IntoIter<[E; N]>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<E, const N: usize> std::fmt::Debug for SmallVecMap<E, N>
where
    E: std::fmt::Debug,
    [E; N]: Array<Item = E>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}

pub type SmallVecPairMap<K, V, const N: usize> = SmallVecMap<(K, V), N>;

#[derive(Debug, Error)]
#[error("insert entry repeat error {old}")]
pub struct InsertEntryRepeatError<Entry> {
    pub old: usize,
    pub new: Entry,
}

#[derive(Debug, Error)]
#[error("from vec entry repeat error {i} {j}")]
pub struct FromVecEntryRepeatError {
    pub i: usize,
    pub j: usize,
}

impl<K, E, const N: usize> SmallVecMap<E, N>
where
    K: PartialEq + Eq,
    E: AsVecMapEntry<K = K>,
    [E; N]: Array<Item = E>,
{
    pub fn clear(&mut self) {
        self.data.clear()
    }

    pub fn take_data(self) -> SmallVec<[E; N]> {
        self.data
    }
    pub fn data(&self) -> &[E] {
        &self.data
    }
    pub fn data_mut(&mut self) -> &mut [E] {
        &mut self.data
    }

    pub fn from_smallvec(data: SmallVec<[E; N]>) -> Result<Self, FromVecEntryRepeatError>
    where
        K: Copy,
    {
        for i in 0..data.len() {
            for j in (i + 1)..data.len() {
                if data[i].key() == data[j].key() {
                    return Err(FromVecEntryRepeatError { i, j });
                }
            }
        }
        Ok(Self { data })
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get_entry(&self, key: K) -> Option<&E>
    where
        K: Copy,
    {
        self.data.iter().find(|entry| entry.key() == key)
    }

    pub fn get_entry_mut(&mut self, key: K) -> Option<&mut E>
    where
        K: Copy,
    {
        self.data.iter_mut().find(|entry| entry.key() == key)
    }

    pub fn iget_entry(&self, key: K) -> Option<(usize, &E)>
    where
        K: Copy,
    {
        self.data
            .iter()
            .enumerate()
            .find(|(_, entry)| entry.key() == key)
    }

    pub fn has(&self, key: K) -> bool
    where
        K: Copy,
    {
        self.data.iter().find(|entry| entry.key() == key).is_some()
    }

    pub fn keys<'a>(&'a self) -> impl Iterator<Item = K> + 'a
    where
        K: Copy,
    {
        self.data.iter().map(|entry| entry.key())
    }

    pub fn get_mut(&mut self, key: K) -> Option<&mut E>
    where
        K: Copy,
    {
        self.data.iter_mut().find(|entry| entry.key() == key)
    }

    pub fn insert_new(&mut self, new: E) -> Result<(), InsertEntryRepeatError<E>>
    where
        K: Copy,
    {
        if self.has(new.key()) {
            let new_key = new.key();
            Err(InsertEntryRepeatError {
                old: self
                    .data
                    .iter()
                    .position(|entry| entry.key() == new_key)
                    .unwrap()
                    .into(),
                new,
            })
        } else {
            self.data.push(new);
            Ok(())
        }
    }

    pub fn insert_new_unchecked(&mut self, new: E)
    where
        K: Copy,
    {
        debug_assert!(!self.has(new.key()));
        self.data.push(new)
    }

    pub fn insert(&mut self, value: E)
    where
        K: Copy,
    {
        if self.has(value.key()) {
            ()
        } else {
            self.data.push(value)
        }
    }
    pub fn insert_from_ref(&mut self, value: &E)
    where
        E: Clone,
        K: Copy,
    {
        if self.has(value.key()) {
            ()
        } else {
            self.data.push(value.clone())
        }
    }

    pub fn position(&self, key: K) -> Option<usize>
    where
        K: Copy,
    {
        self.data.iter().position(|entry| entry.key() == key)
    }

    pub fn extend(&mut self, iter: impl Iterator<Item = E>) -> Result<(), InsertEntryRepeatError<E>>
    where
        K: Copy,
    {
        for v in iter {
            self.insert_new(v)?
        }
        Ok(())
    }

    pub fn extend_from_other(&mut self, other: Self) -> Result<(), InsertEntryRepeatError<E>>
    where
        K: Copy,
    {
        for v in other.data {
            self.insert_new(v)?
        }
        Ok(())
    }

    pub fn extend_from_ref(&mut self, other: &Self)
    where
        E: Clone,
        K: Copy,
    {
        for entry in &other.data {
            self.insert_from_ref(entry)
        }
    }

    pub fn toggle(&mut self, key: K)
    where
        E: DefaultVecMapEntry<K>,
        K: Copy,
    {
        if let Some(position) = self.data.iter().position(|entry| entry.key() == key) {
            self.data.remove(position);
        } else {
            self.data.push(E::default_from_key(key))
        }
    }
}

impl<K, V, const N: usize> SmallVecPairMap<K, V, N>
where
    [(K, V); N]: Array<Item = (K, V)>,
{
    pub fn get_mut_or_insert_default(&mut self, key: K) -> &mut V
    where
        K: Copy + PartialEq,
        V: Default,
    {
        match self.data.iter_mut().find(|(key1, _)| *key1 == key) {
            Some(entry) => unsafe { wild_utils::arb_ref(&mut entry.1) },
            None => {
                self.data.push((key, V::default()));
                &mut unsafe { self.data.last_mut().unwrap_unchecked() }.1
            }
        }
    }

    pub fn get_mut_or_insert_with(&mut self, key: K, f: impl FnOnce() -> V) -> &mut V
    where
        K: Copy + PartialEq,
    {
        match self.data.iter_mut().find(|(key1, _)| *key1 == key) {
            Some(entry) => unsafe { wild_utils::arb_ref(&mut entry.1) },
            None => {
                self.data.push((key, f()));
                &mut unsafe { self.data.last_mut().unwrap_unchecked() }.1
            }
        }
    }
}
impl<K, Entry, const N: usize> From<[Entry; N]> for SmallVecMap<Entry, N>
where
    K: PartialEq + Eq + Copy + std::fmt::Debug,
    Entry: AsVecMapEntry<K = K> + std::fmt::Debug,
    [Entry; N]: Array<Item = Entry>,
{
    fn from(value: [Entry; N]) -> Self {
        let iter: std::array::IntoIter<_, N> = value.into_iter();
        Self::from_iter(iter)
    }
}

impl<K, E, const N: usize> FromIterator<E> for SmallVecMap<E, N>
where
    K: PartialEq + Eq + Copy + std::fmt::Debug,
    E: AsVecMapEntry<K = K> + std::fmt::Debug,
    [E; N]: Array<Item = E>,
{
    fn from_iter<T: IntoIterator<Item = E>>(iter: T) -> Self {
        let mut map = Self::default();
        for v in iter {
            map.insert_new(v).unwrap();
        }
        map
    }
}

impl<K, E, const N: usize> Deref for SmallVecMap<E, N>
where
    K: PartialEq + Eq + Copy + std::fmt::Debug,
    E: AsVecMapEntry<K = K>,
    [E; N]: Array<Item = E>,
{
    type Target = [E];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<E, const N: usize> Default for SmallVecMap<E, N>
where
    [E; N]: Array<Item = E>,
{
    fn default() -> Self {
        Self { data: smallvec![] }
    }
}

impl<K, E, const N: usize> std::ops::Index<K> for SmallVecMap<E, N>
where
    K: PartialEq + Eq + Copy + std::fmt::Debug,
    E: AsVecMapEntry<K = K>,
    [E; N]: Array<Item = E>,
{
    type Output = E;

    fn index(&self, index: K) -> &Self::Output {
        self.get_entry(index).unwrap()
    }
}

impl<K, E, const N: usize> std::ops::IndexMut<K> for SmallVecMap<E, N>
where
    K: PartialEq + Eq + Copy + std::fmt::Debug,
    E: AsVecMapEntry<K = K>,
    [E; N]: Array<Item = E>,
{
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}
