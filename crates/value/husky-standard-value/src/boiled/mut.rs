pub mod slice;

use super::*;

impl<T> Boiled for &mut T
where
    T: Boiled,
{
    type Thawed = ThawedMut<T::Thawed>;

    fn full_type_name() -> std::borrow::Cow<'static, str> {
        format!("&mut {}", T::full_type_name()).into()
    }

    unsafe fn into_thawed(self) -> Self::Thawed {
        unsafe { std::mem::transmute(self) }
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
