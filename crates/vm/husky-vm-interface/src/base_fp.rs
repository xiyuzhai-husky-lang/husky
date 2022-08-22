use crate::*;

pub trait BaseFp: ThinFp {
    type WithContext;

    fn to_raw(self) -> *const ();
}
