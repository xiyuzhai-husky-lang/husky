use crate::*;

/// we use this layout instead of struct to reduce size to `2 * std::mem::size_of::<usize>()`
///
/// stand is used to recover RefMut from snapshot
#[derive(Debug, Clone)]
#[repr(u8)]
pub enum __RegularValueStand {
    Moved = 0,
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
    Intrinsic(Box<dyn __RegularDyn>),
    Box(Box<dyn __RegularDyn>),
    Leash(&'static dyn __RegularDyn),
    SizedRef(Arc<dyn __RegularDyn>),
    SizedRefMut(Box<dyn __RegularDyn>),
    OptionBox(Option<__BoxDynRegularDyn>),
    OptionLeash(Option<&'static dyn __RegularDyn>),
    OptionSizedRef(Option<Arc<dyn __RegularDyn>>),
    OptionSizedRefMut(Option<Box<dyn __RegularDyn>>),
}
