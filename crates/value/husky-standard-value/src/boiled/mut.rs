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
}
