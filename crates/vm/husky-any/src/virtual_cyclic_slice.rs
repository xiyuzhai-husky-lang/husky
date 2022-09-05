use super::*;
use cyclic_slice::CyclicSlice;

#[derive(Debug, Clone, PartialEq)]
pub struct DeprecatedVirtualCyclicSlice<'eval> {
    pub data: CyclicSlice<'eval, __Register<'eval>>,
}

impl<'eval> std::ops::Deref for DeprecatedVirtualCyclicSlice<'eval> {
    type Target = CyclicSlice<'eval, __Register<'eval>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'eval> std::ops::DerefMut for DeprecatedVirtualCyclicSlice<'eval> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'eval> __StaticInfo for DeprecatedVirtualCyclicSlice<'eval> {
    type __StaticSelf = DeprecatedVirtualCyclicSlice<'static>;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "CyclicSlice<Any>".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf
    where
        Self: Sized,
    {
        std::mem::transmute(self)
    }
}

impl<'eval> __Registrable<'eval> for DeprecatedVirtualCyclicSlice<'eval> {
    unsafe fn __to_register(self) -> __Register<'eval> {
        todo!()
    }
}
