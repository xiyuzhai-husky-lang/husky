

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

impl IntegerLikeLiteralData {
    pub fn negative(self) -> Option<Self> {
        Some(match self {
            IntegerLikeLiteralData::UnspecifiedRegular(i) => {
                IntegerLikeLiteralData::UnspecifiedRegular(i)
            }
            IntegerLikeLiteralData::UnspecifiedLarge() => {
                IntegerLikeLiteralData::UnspecifiedLarge()
            }
            IntegerLikeLiteralData::I8(i) => IntegerLikeLiteralData::I8(-i),
            IntegerLikeLiteralData::I16(i) => IntegerLikeLiteralData::I16(-i),
            IntegerLikeLiteralData::I32(i) => IntegerLikeLiteralData::I32(-i),
            IntegerLikeLiteralData::I64(i) => IntegerLikeLiteralData::I64(-i),
            IntegerLikeLiteralData::I128(i) => IntegerLikeLiteralData::I128(-i),
            IntegerLikeLiteralData::ISize(i) => IntegerLikeLiteralData::ISize(-i),
            IntegerLikeLiteralData::R8(_)
            | IntegerLikeLiteralData::R16(_)
            | IntegerLikeLiteralData::R32(_)
            | IntegerLikeLiteralData::R64(_)
            | IntegerLikeLiteralData::R128(_)
            | IntegerLikeLiteralData::RSize(_)
            | IntegerLikeLiteralData::U8(_)
            | IntegerLikeLiteralData::U16(_)
            | IntegerLikeLiteralData::U32(_)
            | IntegerLikeLiteralData::U64(_)
            | IntegerLikeLiteralData::U128(_)
            | IntegerLikeLiteralData::USize(_) => return None,
        })
    }
}
