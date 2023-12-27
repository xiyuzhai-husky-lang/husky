use crate::*;

pub trait __VecX {
    type Element;

    fn ilen(&self) -> i32;

    fn collect_leashes(&'static self) -> Vec<&'static Self::Element>;

    fn pop_with_largest_opt_f32(
        &mut self,
        f: fn(Self::Element) -> Option<f32>,
    ) -> Option<Self::Element>
    where
        Self::Element: Copy;

    fn cyclic_slice_leashed(
        &'static self,
        start: i32,
        end: i32,
    ) -> CyclicSliceLeashed<Self::Element>;
}

impl<T> __VecX for Vec<T> {
    type Element = T;

    fn ilen(&self) -> i32 {
        self.len() as i32
    }

    fn collect_leashes(&'static self) -> Vec<&'static Self::Element> {
        self.iter().collect()
    }

    fn pop_with_largest_opt_f32(&mut self, f: fn(Self::Element) -> Option<f32>) -> Option<T>
    where
        T: Copy,
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

    fn cyclic_slice_leashed(
        &'static self,
        start: i32,
        end: i32,
    ) -> CyclicSliceLeashed<Self::Element> {
        CyclicSliceLeashed::<T>::new(self, start, end)
    }
}
