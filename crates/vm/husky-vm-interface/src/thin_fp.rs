use crate::*;

#[const_trait]
pub trait ThinFp: Copy {
    fn __to_void_pointer(self) -> *const c_void;
}

#[const_trait]
pub trait __BaseThinFp: ~const ThinFp + __StaticInfo {
    type __CtxThinFp: __CtxThinFp;
}

#[const_trait]
pub trait __CtxThinFp: ~const ThinFp + __StaticInfo {}
