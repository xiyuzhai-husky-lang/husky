use std::collections::HashMap;

pub struct Jar<K, T> {
    pub data: HashMap<K, Box<T>>,
}

impl<K, T> Jar<K, T> {
    pub fn get(&self, key: &K) -> Option<&T> {
        todo!()
        // self.data.get(key)
    }

    pub fn alloc(&self, key: K, t: T) -> &T {
        todo!()
        // self.data.insert(key, value);
    }
}
