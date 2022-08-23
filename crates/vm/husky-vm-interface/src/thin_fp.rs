pub trait ThinFp: Copy {}

pub trait BaseFp: ThinFp {
    type __ThinFpWithContext;

    fn __to_void_pointer(self) -> *const ();
}
