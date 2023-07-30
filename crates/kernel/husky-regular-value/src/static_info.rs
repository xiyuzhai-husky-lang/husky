pub trait __StaticInfo {
    type __StandSelf: __StaticInfo<__StandSelf = Self::__StandSelf> + 'static;

    fn __static_type_id__() -> std::any::TypeId {
        std::any::TypeId::of::<Self::__StandSelf>()
    }

    fn __static_typename() -> std::borrow::Cow<'static, str>;

    unsafe fn __transmute_static(self) -> Self::__StandSelf
    where
        Self: Sized;
}

impl<T> __StaticInfo for &T
where
    T: __StaticInfo,
{
    type __StandSelf = &'static T::__StandSelf;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        todo!()
    }

    unsafe fn __transmute_static(self) -> Self::__StandSelf {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> __StaticInfo for &mut T
where
    T: __StaticInfo,
{
    type __StandSelf = &'static mut T::__StandSelf;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        todo!()
    }

    unsafe fn __transmute_static(self) -> Self::__StandSelf {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> __StaticInfo for Option<T>
where
    T: __StaticInfo,
{
    type __StandSelf = Option<T::__StandSelf>;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        format!("?{}", T::__static_typename()).into()
    }

    unsafe fn __transmute_static(self) -> Self::__StandSelf {
        todo!()
    }
}

impl<T> __StaticInfo for Vec<T>
where
    T: __StaticInfo,
{
    type __StandSelf = Vec<T::__StandSelf>;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        format!("?{}", T::__static_typename()).into()
    }

    unsafe fn __transmute_static(self) -> Self::__StandSelf {
        todo!()
    }
}
