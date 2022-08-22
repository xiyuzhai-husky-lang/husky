use cyclic_slice::CyclicSlice;

use super::*;

impl<'a, T> __StaticInfo for CyclicSlice<'a, T>
where
    T: __StaticInfo,
{
    type __StaticSelf = CyclicSlice<'static, T::__StaticSelf>;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        format!("CyclicSlice<{}>", T::__static_typename()).into()
    }

    unsafe fn __as_static(self) -> Self::__StaticSelf {
        std::mem::transmute(self)
    }
}

impl<'a, 'eval, T: __Registrable<'eval>> __Registrable<'eval> for CyclicSlice<'a, T> {
    unsafe fn __to_register(self) -> __Register<'eval> {
        todo!()
    }
}
