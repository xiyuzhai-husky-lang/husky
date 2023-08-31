#[cfg(feature = "vm_support")]
mod incubator;
#[cfg(feature = "vm_support")]
mod snapshot;

#[cfg(feature = "vm_support")]
pub use self::incubator::*;
#[cfg(feature = "vm_support")]
pub use self::snapshot::*;

use crate::*;

pub(crate) const REGULAR_VALUE_SIZE_OVER_I64: usize = 3;

/// we use this layout instead of struct to reduce size to `2 * std::mem::size_of::<usize>()`
#[derive(Debug)]
#[repr(u8)]
pub enum RegularValue {
    Invalid,
    Moved,
    Unit(()),
    Bool(bool),
    Char(char),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
    R8(u8),
    R16(u16),
    R32(u32),
    R64(u64),
    R128(u128),
    RSize(usize),
    F32(f32),
    F64(f64),
    StringLiteral(StringLiteralId),
    /// `Box<T>`
    Box(Box<dyn __RegularStandDyn>),
    // ad hoc
    /// `~T`
    Leash(&'static dyn __RegularStandDyn),
    /// `&T` for T Sized
    SizedRef(*const dyn __RegularStandDyn),
    /// `&mut T` for T Sized
    SizedMut(*mut dyn __RegularStandDyn),
    OptionBox(Option<Box<dyn __RegularStandDyn>>),
    OptionLeash(Option<&'static dyn __RegularStandDyn>),
    OptionSizedRef(Option<*const dyn __RegularStandDyn>),
    OptionSizedMut(Option<*mut dyn __RegularStandDyn>),
    /// T where T is not in above cases
    Intrinsic(Box<dyn __RegularStandDyn>),
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
        std::mem::size_of::<RegularValue>(),
        std::mem::size_of::<[u64; REGULAR_VALUE_SIZE_OVER_I64]>()
    )
}
