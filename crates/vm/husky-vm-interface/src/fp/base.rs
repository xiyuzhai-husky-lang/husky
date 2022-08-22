use crate::*;

pub trait BaseFp<'eval>: ThinFp<'eval> {
    type WithContext: ThinFp<'eval>;
}
