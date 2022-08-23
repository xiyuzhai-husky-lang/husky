use crate::*;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum __VirtualFunction {
    ThickFp(__ResolvedLinkage),
}

// impl __VirtualFunction {
//     pub fn fp(&self) -> *const () {
//         match self {
//             __VirtualFunction::ThickFp(linkage) => linkage.opt_thick_fp.unwrap(),
//         }
//     }
// }

impl __StaticInfo for __VirtualFunction {
    type __StaticSelf = Self;

    fn __static_typename() -> Cow<'static, str> {
        todo!()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf
    where
        Self: Sized,
    {
        self
    }
}

impl<'eval> __Registrable<'eval> for __VirtualFunction {
    unsafe fn __to_register(self) -> __Register<'eval> {
        __Register::new_box(self, &__VIRTUAL_FUNCTION_VTABLE)
    }
}
