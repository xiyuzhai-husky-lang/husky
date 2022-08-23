use crate::__StaticInfo;

pub trait ThinFp: Copy {}

pub trait __GetCtxThinFp<'eval> {
    type __ThinFpWithContext;
}

pub trait BaseThinFp: for<'eval> __GetCtxThinFp<'eval> + ThinFp + __StaticInfo {
    fn __to_void_pointer(self) -> *const ();
}
