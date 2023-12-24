use crate::*;

// #[husky_standard_value::value_conversion]
#[derive(Debug, PartialEq, Eq)]
pub struct CyclicSliceLeashed<T>
where
    T: 'static,
{
    slice: &'static [T],
}
impl<T> __Static for CyclicSliceLeashed<T>
where
    T: __Static,
{
    type Frozen = Self;
    unsafe fn freeze(&self) -> Self::Frozen {
        todo!()
    }
}
impl<T> __Frozen for CyclicSliceLeashed<T>
where
    T: __Static,
{
    type Static = Self;
    type Stand = ();
    fn revive(&self) -> (Option<Self::Stand>, Self::Static) {
        todo!()
    }
}
impl<T> __WeakStatic for CyclicSliceLeashed<T>
where
    T: __Static,
{
    type Static = Self;
    unsafe fn into_static(self) -> Self::Static {
        self
    }
}
impl<T> __FromValue for CyclicSliceLeashed<T>
where
    T: __Static,
{
    fn from_value(value: __Value) -> Self {
        todo!()
    }
}
impl<T> __IntoValue for CyclicSliceLeashed<T>
where
    T: __Static,
{
    fn into_value(self) -> __Value {
        todo!()
    }
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
