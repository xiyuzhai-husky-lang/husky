pub trait __VecX {
    fn ilen(&self) -> i32;
}

impl<T> __VecX for Vec<T> {
    fn ilen(&self) -> i32 {
        self.len() as i32
    }
}
