use crate::*;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum __VirtualFunction {
    ThickFp(__ResolvedLinkage),
}

impl __VirtualFunction {
    // pub fn fp(&self) -> *const c_void {
    //     match self {
    //         __VirtualFunction::ThickFp(linkage) => linkage.opt_thick_fp.unwrap(),
    //     }
    // }

    #[cfg(feature = "thick_fp")]
    pub unsafe fn downcast_thick_fp<F: __BaseThinFp>(&self) -> ThickFp<F> {
        match self {
            __VirtualFunction::ThickFp(linkage) => linkage.opt_thick_fp.downcast_thick_fp(),
        }
    }
}

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

impl __Registrable for __VirtualFunction {
    unsafe fn __to_register(self) -> __RegularValue {
        __RegularValue::new_box(self, &__VIRTUAL_FUNCTION_VTABLE)
    }
}
