use crate::{
    error::{FromVecEntryRepeatError, InsertEntryRepeatError},
    *,
};
use smallvec::{smallvec, Array, SmallVec};

#[derive(PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct SmallVecMap<E, const N: usize>
where
    [E; N]: Array<Item = E>,
{
    entries: SmallVec<[E; N]>,
}

impl<V, const N: usize> std::convert::AsRef<[V]> for SmallVecMap<V, N>
where
    [V; N]: Array<Item = V>,
{
    fn as_ref(&self) -> &[V] {
        &self.entries
    }
}

impl<E, const N: usize> IntoIterator for SmallVecMap<E, N>
where
    [E; N]: Array<Item = E>,
{
    type Item = E;

    type IntoIter = smallvec::IntoIter<[E; N]>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl<E, const N: usize> std::fmt::Debug for SmallVecMap<E, N>
where
    E: std::fmt::Debug,
    [E; N]: Array<Item = E>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.entries.fmt(f)
    }
}

impl<K, E, const N: usize> SmallVecMap<E, N>
where
    K: PartialEq + Eq,
    E: AsVecMapEntry<K = K>,
    [E; N]: Array<Item = E>,
{
    pub fn clear(&mut self) {
        self.entries.clear()
    }

    pub fn take_data(self) -> SmallVec<[E; N]> {
        self.entries
    }
    pub fn data(&self) -> &[E] {
        &self.entries
    }
    pub fn data_mut(&mut self) -> &mut [E] {
        &mut self.entries
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
        Ok(Self { entries: data })
    }

    pub unsafe fn from_smallvec_unchecked(entries: SmallVec<[E; N]>) -> Self {
        Self { entries }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn get_entry(&self, key: K) -> Option<&E>
    where
        K: Copy,
    {
        self.entries.iter().find(|entry| entry.key() == key)
    }

    pub fn get_entry_mut(&mut self, key: K) -> Option<&mut E>
    where
        K: Copy,
    {
        self.entries.iter_mut().find(|entry| entry.key() == key)
    }

    pub fn iget_entry(&self, key: K) -> Option<(usize, &E)>
    where
        K: Copy,
    {
        self.entries
            .iter()
            .enumerate()
            .find(|(_, entry)| entry.key() == key)
    }

    pub fn has(&self, key: K) -> bool
    where
        K: Copy,
    {
        self.entries
            .iter()
            .find(|entry| entry.key() == key)
            .is_some()
    }

    pub fn keys<'a>(&'a self) -> impl Iterator<Item = K> + 'a
    where
        K: Copy,
    {
        self.entries.iter().map(|entry| entry.key())
    }

    pub fn get_mut(&mut self, key: K) -> Option<&mut E>
    where
        K: Copy,
    {
        self.entries.iter_mut().find(|entry| entry.key() == key)
    }

    pub fn insert_new(&mut self, new: E) -> Result<(), InsertEntryRepeatError<E>>
    where
        K: Copy,
    {
        if self.has(new.key()) {
            let new_key = new.key();
            Err(InsertEntryRepeatError {
                old: self
                    .entries
                    .iter()
                    .position(|entry| entry.key() == new_key)
                    .unwrap()
                    .into(),
                new,
            })
        } else {
            self.entries.push(new);
            Ok(())
        }
    }

    pub unsafe fn insert_new_unchecked(&mut self, new: E)
    where
        K: Copy,
    {
        debug_assert!(!self.has(new.key()));
        self.entries.push(new)
    }

    pub fn insert(&mut self, value: E)
    where
        K: Copy,
    {
        if self.has(value.key()) {
            ()
        } else {
            self.entries.push(value)
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
            self.entries.push(value.clone())
        }
    }

    pub fn position(&self, key: K) -> Option<usize>
    where
        K: Copy,
    {
        self.entries.iter().position(|entry| entry.key() == key)
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
        for v in other.entries {
            self.insert_new(v)?
        }
        Ok(())
    }

    pub fn extend_from_ref(&mut self, other: &Self)
    where
        E: Clone,
        K: Copy,
    {
        for entry in &other.entries {
            self.insert_from_ref(entry)
        }
    }

    pub fn toggle(&mut self, key: K)
    where
        E: DefaultVecMapEntry<K>,
        K: Copy,
    {
        if let Some(position) = self.entries.iter().position(|entry| entry.key() == key) {
            self.entries.remove(position);
        } else {
            self.entries.push(E::default_from_key(key))
        }
    }
}

impl<K, V, const N: usize> SmallVecPairMap<K, V, N>
where
    [(K, V); N]: Array<Item = (K, V)>,
{
    pub fn get_value<Borrowed: ?Sized>(&self, key: K) -> Option<&Borrowed>
    where
        K: Eq + Copy,
        V: std::borrow::Borrow<Borrowed>,
    {
        self.get_entry(key).map(|(_, v)| v.borrow())
    }

    #[inline(always)]
    pub fn get_value_mut_or_insert_default(&mut self, key: K) -> &mut V
    where
        K: Copy + PartialEq,
        V: Default,
    {
        match self.entries.iter_mut().find(|(key1, _)| *key1 == key) {
            Some(entry) => unsafe { wild_utils::arb_lifetime_mut(&mut entry.1) },
            None => {
                self.entries.push((key, V::default()));
                &mut unsafe { self.entries.last_mut().unwrap_unchecked() }.1
            }
        }
    }

    #[inline(always)]
    pub fn get_value_mut_or_insert_with(&mut self, key: K, f: impl FnOnce() -> V) -> &mut V
    where
        K: Copy + PartialEq,
    {
        match self.entries.iter_mut().find(|(key1, _)| *key1 == key) {
            Some(entry) => unsafe { wild_utils::arb_lifetime_mut(&mut entry.1) },
            None => {
                self.entries.push((key, f()));
                &mut unsafe { self.entries.last_mut().unwrap_unchecked() }.1
            }
        }
    }

    #[inline(always)]
    pub fn update_value_or_insert(&mut self, key: K, update: impl FnOnce(&mut V), v: V)
    where
        K: Copy + PartialEq,
    {
        match self.entries.iter_mut().find(|(key1, _)| *key1 == key) {
            Some(entry) => update(&mut entry.1),
            None => self.entries.push((key, v)),
        }
    }

    #[inline(always)]
    pub fn update_value_or_insert_with(
        &mut self,
        key: K,
        update: impl FnOnce(&mut V),
        f: impl FnOnce() -> V,
    ) where
        K: Copy + PartialEq,
    {
        match self.entries.iter_mut().find(|(key1, _)| *key1 == key) {
            Some(entry) => update(&mut entry.1),
            None => self.entries.push((key, f())),
        }
    }

    #[inline(always)]
    pub fn map_collect<U>(&self, f: impl Fn(&V) -> U) -> SmallVecPairMap<K, U, N>
    where
        K: Copy,
        [(K, U); N]: Array<Item = (K, U)>,
    {
        SmallVecPairMap {
            entries: self.entries.iter().map(|(k, v)| (*k, f(v))).collect(),
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
            debug_assert!(!map.has(v.key()));
            unsafe { map.insert_new_unchecked(v) }
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
        &self.entries
    }
}

impl<E, const N: usize> Default for SmallVecMap<E, N>
where
    [E; N]: Array<Item = E>,
{
    fn default() -> Self {
        Self {
            entries: smallvec![],
        }
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

pub type SmallVecPairMap<K, V, const N: usize> = SmallVecMap<(K, V), N>;
