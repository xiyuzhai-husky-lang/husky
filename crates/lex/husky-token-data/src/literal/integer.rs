
use std::ops::Neg;

/// integer-like means it looks like an integer
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum IntegerLikeLiteralData {
    /// unspecified but within i128
    UnspecifiedRegular(i128),
    UnspecifiedLarge(),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    R8(u8),
    R16(u16),
    R32(u32),
    R64(u64),
    R128(u128),
    RSize(usize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    USize(usize),
}

impl Neg for IntegerLikeLiteralData {
    type Output = Self;

    fn neg(self) -> Self::Output {
        todo!()
    }
}
