pub mod slice;

use super::*;

impl<T> Boiled for &T
where
    T: Boiled,
{
    type Thawed = ThawedRef<T::Thawed>;

    fn full_type_name() -> std::borrow::Cow<'static, str> {
        format!("&{}", T::full_type_name()).into()
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
}
