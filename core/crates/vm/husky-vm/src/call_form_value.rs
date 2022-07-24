use crate::*;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct __CallFormValue {
    pub opt_linkage: Option<__Linkage>,
}

impl __StaticInfo for __CallFormValue {
    type __StaticSelf = Self;

    fn __static_type_name__() -> Cow<'static, str> {
        todo!()
    }
}

impl __Registrable for __CallFormValue {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        todo!()
    }
}
