use super::*;
use std::ops::ControlFlow;

impl<C, B> Boiled for ControlFlow<C, B>
where
    C: Boiled,
    B: Boiled,
{
    type Thawed = ControlFlow<C::Thawed, B::Thawed>;

    unsafe fn into_thawed(self) -> Self::Thawed
    where
        Self: Sized,
    {
        todo!()
    }

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
}
