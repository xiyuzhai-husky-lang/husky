use crate::*;
pub trait __VecX<T> {
    fn ilen(&self) -> i32;

    fn __call__(__variadics: Vec<T>) -> Self;

    fn cyclic_slice<'a>(&'a self, start: i32, end: i32) -> CyclicSlice<'a, T>;

    fn popx(&mut self) -> T;

    fn firstx(&self) -> &T;

    fn firstx_mut(&mut self) -> &mut T;

    fn lastx(&self) -> &T;

    fn lastx_mut(&mut self) -> &mut T;
}

impl<T> __VecX<T> for Vec<T> {
    fn ilen(&self) -> i32 {
        self.len() as i32
    }

    fn __call__(__variadics: Vec<T>) -> Self {
        __variadics
    }

    fn cyclic_slice<'a>(&'a self, start: i32, end: i32) -> CyclicSlice<'a, T> {
        CyclicSlice::<T> {
            start,
            end,
            total: self.as_slice(),
        }
    }

    fn popx(&mut self) -> T {
        self.pop().unwrap()
    }

    fn firstx(&self) -> &T {
        self.first().unwrap()
    }

    fn firstx_mut(&mut self) -> &mut T {
        self.first_mut().unwrap()
    }

    fn lastx(&self) -> &T {
        self.last().unwrap()
    }

    fn lastx_mut(&mut self) -> &mut T {
        self.last_mut().unwrap()
    }
}
