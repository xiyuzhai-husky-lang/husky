pub trait IntoStatic {
    type Static: IntoStatic<Static = Self::Static> + 'static;

    fn type_id() -> std::any::TypeId {
        std::any::TypeId::of::<Self::Static>()
    }

    fn full_typename() -> std::borrow::Cow<'static, str>;

    /// should call `std::mem::transmute` under the hood
    unsafe fn into_static(self) -> Self::Static
    where
        Self: Sized;
}

impl<T> IntoStatic for &T
where
    T: IntoStatic,
{
    type Static = &'static T::Static;

    fn full_typename() -> std::borrow::Cow<'static, str> {
        format!("&{}", T::full_typename()).into()
    }

    unsafe fn into_static(self) -> Self::Static {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> IntoStatic for &mut T
where
    T: IntoStatic,
{
    type Static = &'static mut T::Static;

    fn full_typename() -> std::borrow::Cow<'static, str> {
        format!("&mut {}", T::full_typename()).into()
    }

    unsafe fn into_static(self) -> Self::Static {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> IntoStatic for Option<T>
where
    T: IntoStatic,
{
    type Static = Option<T::Static>;

    fn full_typename() -> std::borrow::Cow<'static, str> {
        format!("?{}", T::full_typename()).into()
    }

    unsafe fn into_static(self) -> Self::Static {
        self.map(|slf| slf.into_static())
    }
}

impl<T> IntoStatic for Vec<T>
where
    T: IntoStatic,
{
    type Static = Vec<T::Static>;

    fn full_typename() -> std::borrow::Cow<'static, str> {
        format!("?{}", T::full_typename()).into()
    }

    unsafe fn into_static(self) -> Self::Static {
        unsafe { std::mem::transmute(self) }
    }
}
