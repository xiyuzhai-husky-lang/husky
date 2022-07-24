use cyclic_slice::CyclicSlice;

use super::*;

impl<'a, T> __StaticInfo for CyclicSlice<'a, T>
where
    T: __StaticInfo,
{
    type __StaticSelf = CyclicSlice<'static, T::__StaticSelf>;
    fn __static_type_name__() -> std::borrow::Cow<'static, str> {
        format!("CyclicSlice<{}>", T::__static_type_name__()).into()
    }
}

impl<'a, T: __Registrable> __Registrable for CyclicSlice<'a, T> {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        todo!()
    }
}
