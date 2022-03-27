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

    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        self.data
            .iter_mut()
            .find(|entry| entry.0 == key)
            .map(|entry| &mut entry.1)
    }

    pub fn insert_new(&mut self, key: K, value: V) {
        if let Some(_) = self.data.iter_mut().find(|entry| entry.0 == key) {
            panic!()
        } else {
            self.data.push((key, value))
        }
    }

    pub fn position(&self, key: K) -> Option<usize> {
        self.data.iter().position(|entry| entry.0 == key)
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

impl<K, V> std::ops::Index<K> for VecMap<K, V>
where
    K: PartialEq + Eq,
{
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        self.get(index).unwrap()
    }
}
