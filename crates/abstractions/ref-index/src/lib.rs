pub trait RefIndex<'a, I>: Copy {
    type Output: ?Sized;

    fn ref_index(self, index: I) -> &'a Self::Output;
}
