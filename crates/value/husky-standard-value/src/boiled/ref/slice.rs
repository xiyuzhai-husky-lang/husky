use crate::thawed::r#ref::slice::ThawedSliceRef;

use super::*;

impl<T> Boiled for &[T]
where
    T: Boiled,
{
    type Thawed = ThawedSliceRef<T::Thawed>; // ad hoc

    unsafe fn from_thawed(thawed: Self::Thawed) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    unsafe fn into_thawed(self) -> Self::Thawed
    where
        Self: Sized,
    {
        todo!()
    }
}
