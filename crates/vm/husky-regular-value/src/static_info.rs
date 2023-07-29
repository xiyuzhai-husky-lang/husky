pub trait __StaticInfo {
    type __StaticSelf: __StaticInfo<__StaticSelf = Self::__StaticSelf> + 'static;

    fn __static_type_id__() -> std::any::TypeId {
        std::any::TypeId::of::<Self::__StaticSelf>()
    }

    fn __static_typename() -> std::borrow::Cow<'static, str>;

    unsafe fn __transmute_static(self) -> Self::__StaticSelf
    where
        Self: Sized;
}

impl<T> __StaticInfo for &T
where
    T: __StaticInfo,
{
    type __StaticSelf = &'static T::__StaticSelf;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        todo!()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> __StaticInfo for &mut T
where
    T: __StaticInfo,
{
    type __StaticSelf = &'static mut T::__StaticSelf;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        todo!()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> __StaticInfo for Option<T>
where
    T: __StaticInfo,
{
    type __StaticSelf = Option<T::__StaticSelf>;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        format!("?{}", T::__static_typename()).into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        todo!()
    }
}

impl<T> __StaticInfo for Vec<T>
where
    T: __StaticInfo,
{
    type __StaticSelf = Vec<T::__StaticSelf>;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        format!("?{}", T::__static_typename()).into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        todo!()
    }
}
