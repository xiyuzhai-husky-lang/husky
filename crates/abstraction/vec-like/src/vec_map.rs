use husky_display_utils::{HuskyDisplay, HuskyDisplayConfig};

use crate::*;

pub trait AsVecMapEntry {
    type K;
    fn key(&self) -> Self::K
    where
        Self::K: Copy;
    fn key_ref(&self) -> &Self::K;
}

impl<K, T> AsVecMapEntry for &T
where
    T: AsVecMapEntry<K = K>,
{
    type K = K;
    fn key(&self) -> K
    where
        K: Copy,
    {
        <T as AsVecMapEntry>::key(self)
    }

    fn key_ref(&self) -> &K {
        <T as AsVecMapEntry>::key_ref(self)
    }
}

pub trait DefaultVecMapEntry<K> {
    fn default_from_key(key: K) -> Self;
}

impl<K, T> DefaultVecMapEntry<K> for (K, T)
where
    T: Default,
{
    fn default_from_key(key: K) -> Self {
        (key, T::default())
    }
}

impl<K, T> AsVecMapEntry for (K, T)
where
    K: PartialEq + Eq + std::fmt::Debug,
{
    type K = K;

    fn key(&self) -> K
    where
        K: Copy,
    {
        self.0
    }

    fn key_ref(&self) -> &K {
        &self.0
    }
}

impl<K, T> AsVecMapEntry for Arc<T>
where
    T: AsVecMapEntry<K = K>,
{
    type K = K;
    fn key(&self) -> K
    where
        K: Copy,
    {
        (**self).key()
    }

    fn key_ref(&self) -> &K {
        (**self).key_ref()
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct VecMap<V>
where
    V: AsVecMapEntry,
{
    entries: Vec<V>,
}

pub trait VecMapGetEntry<V>
where
    V: AsVecMapEntry,
    V::K: Copy,
{
    fn get_entry<'a>(&'a self, k: <V as AsVecMapEntry>::K) -> Option<&'a V>;
}

impl<V> VecMapGetEntry<V> for [V]
where
    V: AsVecMapEntry,
    V::K: Copy + Eq,
{
    fn get_entry<'a>(&'a self, k: <V as AsVecMapEntry>::K) -> Option<&'a V> {
        self.iter().find(|v| v.key() == k)
    }
}

impl<K, V> IntoIterator for VecMap<V>
where
    K: PartialEq + Eq + std::fmt::Debug,
    V: AsVecMapEntry<K = K>,
{
    type Item = V;

    type IntoIter = std::vec::IntoIter<V>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl<K, V> std::fmt::Debug for VecMap<V>
where
    K: PartialEq + Eq + std::fmt::Debug,
    V: AsVecMapEntry<K = K> + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.entries.fmt(f)
    }
}

pub type VecPairMap<K, V> = VecMap<(K, V)>;

#[derive(Debug)]
pub struct EntryRepeatError<Entry> {
    pub old: Entry,
    pub new: Entry,
}

impl<K, Entry> VecMap<Entry>
where
    K: PartialEq + Eq,
    Entry: AsVecMapEntry<K = K>,
{
    pub fn clear(&mut self) {
        self.entries.clear()
    }

    pub fn take_data(self) -> Vec<Entry> {
        self.entries
    }
    pub fn data(&self) -> &[Entry] {
        &self.entries
    }
    pub fn data_mut(&mut self) -> &mut [Entry] {
        &mut self.entries
    }

    pub fn from_vec(mut data: Vec<Entry>) -> Result<Self, EntryRepeatError<Entry>>
    where
        K: Copy,
    {
        for i in 0..data.len() {
            for j in (i + 1)..data.len() {
                if data[i].key() == data[j].key() {
                    let new = loop {
                        let entry = data.pop().unwrap();
                        if data.len() == j {
                            break entry;
                        }
                    };
                    let old = loop {
                        let entry = data.pop().unwrap();
                        if data.len() == i {
                            break entry;
                        }
                    };
                    return Err(EntryRepeatError { old, new });
                }
            }
        }
        Ok(Self { entries: data })
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn get_entry(&self, key: K) -> Option<&Entry>
    where
        K: Copy,
    {
        self.entries.iter().find(|entry| entry.key() == key)
    }

    pub fn iget_entry(&self, key: K) -> Option<(usize, &Entry)>
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

    pub fn get_mut(&mut self, key: K) -> Option<&mut Entry>
    where
        K: Copy,
    {
        self.entries.iter_mut().find(|entry| entry.key() == key)
    }

    pub fn insert_new(&mut self, new: Entry) -> Result<(), EntryRepeatError<Entry>>
    where
        K: Copy,
    {
        if self.has(new.key()) {
            let old = loop {
                let entry = self.entries.pop().unwrap();
                if entry.key() == new.key() {
                    break entry;
                }
            };
            Err(EntryRepeatError { old, new })
        } else {
            self.entries.push(new);
            Ok(())
        }
    }

    pub fn insert(&mut self, value: Entry)
    where
        K: Copy,
    {
        if self.has(value.key()) {
            ()
        } else {
            self.entries.push(value)
        }
    }
    pub fn insert_from_ref(&mut self, value: &Entry)
    where
        Entry: Clone,
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

    pub fn extend(
        &mut self,
        iter: impl Iterator<Item = Entry>,
    ) -> Result<(), EntryRepeatError<Entry>>
    where
        K: Copy,
    {
        for v in iter {
            self.insert_new(v)?
        }
        Ok(())
    }

    pub fn extend_from_other(&mut self, other: Self) -> Result<(), EntryRepeatError<Entry>>
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
        Entry: Clone,
        K: Copy,
    {
        for entry in &other.entries {
            self.insert_from_ref(entry)
        }
    }

    pub fn toggle(&mut self, key: K)
    where
        Entry: DefaultVecMapEntry<K>,
        K: Copy,
    {
        if let Some(position) = self.entries.iter().position(|entry| entry.key() == key) {
            self.entries.remove(position);
        } else {
            self.entries.push(Entry::default_from_key(key))
        }
    }
}

impl<K, Entry> FromIterator<Entry> for VecMap<Entry>
where
    K: PartialEq + Eq + Copy + std::fmt::Debug,
    Entry: AsVecMapEntry<K = K> + std::fmt::Debug,
{
    fn from_iter<T: IntoIterator<Item = Entry>>(iter: T) -> Self {
        let mut map = Self::default();
        for v in iter {
            map.insert_new(v).unwrap();
        }
        map
    }
}

impl<K, V> Deref for VecMap<V>
where
    K: PartialEq + Eq + Copy + std::fmt::Debug,
    V: AsVecMapEntry<K = K>,
{
    type Target = [V];

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl<K, V> Default for VecMap<V>
where
    K: PartialEq + Eq + Copy + std::fmt::Debug,
    V: AsVecMapEntry<K = K>,
{
    fn default() -> Self {
        Self { entries: vec![] }
    }
}

impl<K, V> std::ops::Index<K> for VecMap<V>
where
    K: PartialEq + Eq + Copy + std::fmt::Debug,
    V: AsVecMapEntry<K = K>,
{
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        self.get_entry(index).unwrap()
    }
}

impl<K, V> std::ops::IndexMut<K> for VecMap<V>
where
    K: PartialEq + Eq + Copy + std::fmt::Debug,
    V: AsVecMapEntry<K = K>,
{
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl<K, V> HuskyDisplay for VecMap<V>
where
    K: PartialEq + Eq + Copy + std::fmt::Debug,
    V: AsVecMapEntry<K = K> + HuskyDisplay,
{
    fn write_inherent(&self, config: HuskyDisplayConfig, result: &mut String) {
        for entry in &self.entries {
            result.push_str("    ");
            entry.write_inherent(config.indented(), result);
            result.push('\n')
        }
    }
}
