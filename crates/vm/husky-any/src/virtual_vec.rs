use super::*;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct DeprecatedVirtualVec<'eval> {
    data: Vec<__Register<'eval>>,
}

impl<'eval> DeprecatedVirtualVec<'eval> {
    pub fn new(data: Vec<__Register<'eval>>) -> Self {
        Self { data }
    }
}

impl<'eval> std::ops::Deref for DeprecatedVirtualVec<'eval> {
    type Target = Vec<__Register<'eval>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'eval> std::ops::DerefMut for DeprecatedVirtualVec<'eval> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'eval> __StaticInfo for DeprecatedVirtualVec<'eval> {
    type __StaticSelf = DeprecatedVirtualVec<'static>;

    fn __static_typename() -> Cow<'static, str> {
        "[]Any".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf
    where
        Self: Sized,
    {
        std::mem::transmute(self)
    }
}

impl<'eval> __Registrable<'eval> for DeprecatedVirtualVec<'eval> {
    unsafe fn __to_register(self) -> __Register<'eval> {
        __Register::new_box(self, &__DEPRECATED_VIRTUAL_VEC_VTABLE)
    }
}
