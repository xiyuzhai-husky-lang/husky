use super::*;
use crate::thawed::r#ref::slice::ThawedSliceRef;

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

    #[inline]
    unsafe fn from_thawed_ref(thawed_ref: &Self::Thawed) -> &Self {
        std::mem::transmute(thawed_ref)
    }

    unsafe fn into_thawed(self) -> Self::Thawed
    where
        Self: Sized,
    {
        todo!()
    }
}
