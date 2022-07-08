use indexmap::IndexMap;
use serde::{ser::SerializeSeq, Serialize};
use std::{
    hash::Hash,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonListMap<K, T>
where
    K: Hash + PartialEq + Eq,
{
    data: IndexMap<K, T>,
}

impl<K, T> Default for JsonListMap<K, T>
where
    K: Hash + PartialEq + Eq + Serialize,
    T: Serialize,
{
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl<K, T> Serialize for JsonListMap<K, T>
where
    K: Hash + PartialEq + Eq + Serialize,
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for pair in self.data.iter() {
            seq.serialize_element(&pair)?
        }
        seq.end()
    }
}

impl<K, T> Deref for JsonListMap<K, T>
where
    K: Hash + PartialEq + Eq,
{
    type Target = IndexMap<K, T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<K, T> DerefMut for JsonListMap<K, T>
where
    K: Hash + PartialEq + Eq,
{
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        &mut self.data
    }
}

#[test]
fn test_it() {
    let mut m: JsonListMap<i32, bool> = Default::default();
    m.insert(0, true);
    let _ = serde_json::to_string(&m);
}
