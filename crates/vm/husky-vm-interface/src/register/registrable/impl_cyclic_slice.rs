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

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        std::mem::transmute(self)
    }
}

impl<'a, T: __Registrable> __Registrable for CyclicSlice<'a, T> {
    unsafe fn __to_register(self) -> __RegularValue {
        todo!()
    }
}
