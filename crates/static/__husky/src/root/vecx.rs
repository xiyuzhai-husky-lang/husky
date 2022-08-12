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

    fn collect_refs(&self) -> Vec<&T>;

    fn pop_with_opt_largest_f32_copyable(&mut self, f: fn(T) -> Option<f32>) -> Option<T>
    where
        T: Copy + std::fmt::Debug;

    fn pop_with_opt_largest_f32_borrow(&mut self, f: fn(&T) -> Option<f32>) -> Option<T>;
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

    fn collect_refs(&self) -> Vec<&T> {
        self.iter().collect()
    }

    fn pop_with_opt_largest_f32_copyable(&mut self, f: fn(T) -> Option<f32>) -> Option<T>
    where
        T: Copy + std::fmt::Debug,
    {
        let mut imax = None;
        let mut vmax = f32::MIN;
        for i in 0..self.len() {
            if let Some(v) = f(self[i]) {
                if v > vmax {
                    imax = Some(i);
                    vmax = v
                }
            }
        }
        imax.map(|imax| self.remove(imax))
    }

    fn pop_with_opt_largest_f32_borrow(&mut self, f: fn(&T) -> Option<f32>) -> Option<T> {
        let mut imax = None;
        let mut vmax = f32::MIN;
        for i in 0..self.len() {
            if let Some(v) = f(&self[i]) {
                if v > vmax {
                    imax = Some(i);
                    vmax = v
                }
            }
        }
        imax.map(|imax| self.remove(imax))
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
