use crate::__StaticInfo;

pub trait ThinFp: Copy {
    fn needs_context() -> bool;

    fn __to_void_pointer(self) -> *const ();
}

pub trait __BaseThinFp: ~const ThinFp + __StaticInfo {
    type __ThinFpWithContext: __CtxThinFp;
}

pub trait __CtxThinFp: ~const ThinFp + __StaticInfo {}
