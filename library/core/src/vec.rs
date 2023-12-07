pub trait __VecX {
    type Element;

    fn ilen(&self) -> i32;

    fn collect_leashes(&'static self) -> Vec<&'static Self::Element>;

    fn pop_with_largest_opt_f32(
        &mut self,
        f: fn(Self::Element) -> Option<f32>,
    ) -> Option<Self::Element>;
}

impl<T> __VecX for Vec<T> {
    type Element = T;

    fn ilen(&self) -> i32 {
        self.len() as i32
    }

    fn collect_leashes(&'static self) -> Vec<&'static Self::Element> {
        self.iter().collect()
    }

    fn pop_with_largest_opt_f32(&mut self, f: fn(Self::Element) -> Option<f32>) -> Option<T> {
        todo!()
    }
}
