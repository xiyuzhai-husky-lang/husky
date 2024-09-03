use crate::*;
use std::panic::{RefUnwindSafe, UnwindSafe};

pub trait __VecX:
    __Thawed + std::fmt::Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static
{
    type Element: __Thawed + std::fmt::Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static;

    fn ilen(&self) -> i32;

    // todo: change this to associated function with the first argument being of type Leash<[Self::Element]>
    fn collect_leashes(__self: Leash<Self>) -> Vec<Leash<Self::Element>>;

    fn pop_with_largest_opt_f32(
        &mut self,
        f: fn(Self::Element) -> Option<f32>,
    ) -> Option<Self::Element>
    where
        Self::Element: Copy;

    fn cyclic_slice_leashed(
        __self: Leash<Self>,
        start: i32,
        end: i32,
    ) -> CyclicSliceLeashed<Self::Element>;
}

impl<T> __VecX for Vec<T>
where
    T: __Thawed + std::fmt::Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static,
{
    type Element = T;

    fn ilen(&self) -> i32 {
        self.len() as i32
    }

    fn collect_leashes(__self: Leash<Self>) -> Vec<Leash<Self::Element>> {
        __self.deleash().iter().map(Leash::new).collect()
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
        __self: Leash<Self>,
        start: i32,
        end: i32,
    ) -> CyclicSliceLeashed<Self::Element> {
        CyclicSliceLeashed::<T>::new(__self.deleash(), start, end)
    }
}
