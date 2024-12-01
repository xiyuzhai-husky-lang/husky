pub struct Jar0<T> {
    pub data: T,
}

impl<T> Jar0<T> {
    #[inline(always)]
    pub fn get(&self, _key: &()) -> Option<&T> {
        todo!()
        // &self.data
    }

    #[inline(always)]
    pub fn alloc(&self, _key: (), value: T) -> &T {
        todo!()
    }
}
