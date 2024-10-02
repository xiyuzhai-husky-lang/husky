use super::*;

impl<T> Boiled for Option<T>
where
    T: Boiled,
{
    type Thawed = Option<T::Thawed>;

    fn full_type_name() -> std::borrow::Cow<'static, str> {
        format!("?{}", T::full_type_name()).into()
    }

    unsafe fn into_thawed(self) -> Self::Thawed {
        self.map(|slf| slf.into_thawed())
    }

    unsafe fn from_thawed(thawed: Self::Thawed) -> Self
    where
        Self: Sized,
    {
        todo!()
    }
}
