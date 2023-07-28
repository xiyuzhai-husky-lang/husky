use super::*;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct DeprecatedVirtualVec {
    data: Vec<__RegularValue>,
}

impl DeprecatedVirtualVec {
    pub fn new(data: Vec<__RegularValue>) -> Self {
        Self { data }
    }
}

impl std::ops::Deref for DeprecatedVirtualVec {
    type Target = Vec<__RegularValue>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl std::ops::DerefMut for DeprecatedVirtualVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl __StaticInfo for DeprecatedVirtualVec {
    type __StaticSelf = DeprecatedVirtualVec;

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

impl __Registrable for DeprecatedVirtualVec {
    unsafe fn __to_register(self) -> __RegularValue {
        __RegularValue::new_box(self, &__DEPRECATED_VIRTUAL_VEC_VTABLE)
    }
}
