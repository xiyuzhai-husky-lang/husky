use crate::thawed::r#mut::slice::ThawedSliceMut;

use super::*;

impl<T> Boiled for &mut [T]
where
    T: Boiled,
{
    type Thawed = ThawedSliceMut<T::Thawed>; // ad hoc

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
