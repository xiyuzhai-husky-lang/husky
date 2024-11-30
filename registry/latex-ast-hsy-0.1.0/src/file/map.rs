use super::*;

pub struct LxFileMap<V> {
    values: Vec<V>,
}

impl<V> std::ops::Index<LxFileIdx> for LxFileMap<V> {
    type Output = V;

    fn index(&self, idx: LxFileIdx) -> &Self::Output {
        &self.values[idx.index()]
    }
}

impl<V> std::ops::Deref for LxFileMap<V> {
    type Target = Vec<V>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl LxFileStorage {
    pub fn file_map<'a, V>(&'a self, f: impl Fn(usize, &'a str) -> V) -> LxFileMap<V> {
        LxFileMap {
            values: self
                .files
                .iter()
                .enumerate()
                .map(|(i, s)| f(i, s))
                .collect(),
        }
    }
}
