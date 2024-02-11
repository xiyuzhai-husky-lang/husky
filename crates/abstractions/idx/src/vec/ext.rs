#[derive(Clone, Debug)]
pub struct ExtVec<T> {
    pub vec: Vec<T>,
    pub limit: usize,
}
impl<T> Default for ExtVec<T> {
    fn default() -> Self {
        Self {
            vec: vec![],
            limit: 0,
        }
    }
}
impl<T> std::ops::Deref for ExtVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.vec[..self.limit]
    }
}
impl<T> std::ops::DerefMut for ExtVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec[..self.limit]
    }
}
impl<T> ExtVec<T> {
    pub fn push(&mut self, t: T) {
        assert!(self.vec.len() == self.limit);
        self.vec.push(t);
        self.up();
    }
    pub fn push_ext(&mut self, t: T) {
        self.vec.push(t)
    }
    pub fn up(&mut self) {
        self.limit = self.vec.len()
    }
    pub fn len(&self) -> usize {
        self.limit
    }
    pub fn ext_len(&self) -> usize {
        self.vec.len()
    }
}
