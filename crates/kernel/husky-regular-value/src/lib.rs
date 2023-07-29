#[cfg(feature = "constant")]
mod constant;
mod regular;
mod snapshot;
mod static_info;

use std::num::NonZeroU32;

#[cfg(feature = "constant")]
pub use self::constant::*;
pub use self::regular::*;
pub use self::snapshot::*;
pub use self::static_info::*;

#[cfg(feature = "constant")]
use husky_term_prelude::*;

const REGULAR_VALUE_SIZE_OVER_I64: usize = 3;

/// we use this layout instead of struct to reduce size to `2 * std::mem::size_of::<usize>()`
#[derive(Debug)]
#[repr(u8)]
pub enum __RegularValue {
    Unit(()),
    Bool(bool),
    Char(char),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(u128),
    R8(u8),
    R16(u16),
    R32(u32),
    R64(u64),
    R128(u128),
    RSize(u128),
    F32(f32),
    F64(f64),
    StringLiteral(StringLiteralId),
    /// `Box<T>`
    Box(Box<dyn __RegularDyn>),
    // ad hoc
    /// `~T`
    Leash(&'static dyn __RegularDyn),
    /// `&T` for T Sized
    SizedRef(*const dyn __RegularDyn),
    /// `&mut T` for T Sized
    SizedRefMut(*mut dyn __RegularDyn),
    OptionBox(Option<Box<dyn __RegularDyn>>),
    OptionLeash(Option<&'static dyn __RegularDyn>),
    OptionSizedRef(Option<*const dyn __RegularDyn>),
    OptionSizedRefMut(Option<*const dyn __RegularDyn>),
    /// T where T is not in above cases
    Intrinsic(Box<dyn __RegularDyn>),
    Moved,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct StringLiteralId(NonZeroU32);

#[cfg(feature = "constant")]
impl From<StringLiteral> for StringLiteralId {
    fn from(lit: StringLiteral) -> Self {
        unsafe { std::mem::transmute(lit) }
    }
}

#[test]
fn regular_value_size_works() {
    assert_eq!(
        std::mem::size_of::<__RegularValue>(),
        std::mem::size_of::<[u64; REGULAR_VALUE_SIZE_OVER_I64]>()
    )
}
