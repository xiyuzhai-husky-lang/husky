use crate::__StaticInfo;

pub trait ThinFp: Copy {}

pub trait BaseFp: ThinFp + __StaticInfo {
    type __ThinFpWithContext;

    fn __to_void_pointer(self) -> *const ();
}
