use crate::*;

pub trait ThinFp: Copy {
    fn __to_void_pointer(self) -> *const c_void;
}

pub trait __BaseThinFp: ~const ThinFp + __StaticInfo {
    type __CtxThinFp: __CtxThinFp;
}

pub trait __CtxThinFp: ~const ThinFp + __StaticInfo {}
