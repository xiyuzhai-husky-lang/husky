use super::*;

impl<T> Boiled for Vec<T>
where
    T: Boiled,
{
    type Thawed = Vec<T::Thawed>;

    fn full_type_name() -> std::borrow::Cow<'static, str> {
        format!("?{}", T::full_type_name()).into()
    }

    unsafe fn into_thawed(self) -> Self::Thawed {
        unsafe { std::mem::transmute(self) }
    }
}
