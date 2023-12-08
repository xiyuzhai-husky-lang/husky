#[derive(Debug, PartialEq, Eq)]
pub struct CyclicSliceLeashed<T>
where
    T: 'static,
{
    slice: &'static [T],
}

impl<T> Clone for CyclicSliceLeashed<T> {
    fn clone(&self) -> Self {
        Self { slice: self.slice }
    }
}

impl<T> Copy for CyclicSliceLeashed<T> {}

impl<T> CyclicSliceLeashed<T> {
    pub fn first(self) -> Option<&'static T> {
        todo!()
    }

    pub fn last(self) -> Option<&'static T> {
        todo!()
    }

    pub fn start(self) -> i32 {
        todo!()
    }

    pub fn end(self) -> i32 {
        todo!()
    }
}

impl<T> std::ops::Index<usize> for CyclicSliceLeashed<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        todo!()
    }
}
