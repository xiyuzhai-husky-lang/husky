use crate::*;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum __VirtualFunction {
    Fp(__LinkageFp),
}

impl __StaticInfo for __VirtualFunction {
    type __StaticSelf = Self;

    fn __static_typename() -> Cow<'static, str> {
        todo!()
    }
}

impl __Registrable for __VirtualFunction {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        __Register::new_box(self, &__VIRTUAL_FUNCTION_VTABLE)
    }

    fn __copy__(&self) -> Self {
        self.clone()
    }
}
