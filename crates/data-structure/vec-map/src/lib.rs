use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VecMap<K, V>
where
    K: PartialEq + Eq,
{
    data: Vec<(K, V)>,
}

impl<K, V> VecMap<K, V>
where
    K: PartialEq + Eq,
{
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, key: K) -> Option<&V> {
        self.data
            .iter()
            .find(|entry| entry.0 == key)
            .map(|entry| &entry.1)
    }

    pub fn insert_new(&mut self, key: K, value: V) {
        if let Some(_) = self.data.iter_mut().find(|entry| entry.0 == key) {
            panic!()
        } else {
            self.data.push((key, value))
        }
    }
}

impl<K, V> Deref for VecMap<K, V>
where
    K: PartialEq + Eq,
{
    type Target = [(K, V)];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<K, V> Default for VecMap<K, V>
where
    K: PartialEq + Eq,
{
    fn default() -> Self {
        Self { data: vec![] }
    }
}
