/// integer-like means it looks like an integer
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum IntegerLikeLiteralTokenData {
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

impl IntegerLikeLiteralTokenData {
    pub fn negative(self) -> Option<Self> {
        Some(match self {
            IntegerLikeLiteralTokenData::UnspecifiedRegular(i) => {
                IntegerLikeLiteralTokenData::UnspecifiedRegular(i)
            }
            IntegerLikeLiteralTokenData::UnspecifiedLarge() => {
                IntegerLikeLiteralTokenData::UnspecifiedLarge()
            }
            IntegerLikeLiteralTokenData::I8(i) => IntegerLikeLiteralTokenData::I8(-i),
            IntegerLikeLiteralTokenData::I16(i) => IntegerLikeLiteralTokenData::I16(-i),
            IntegerLikeLiteralTokenData::I32(i) => IntegerLikeLiteralTokenData::I32(-i),
            IntegerLikeLiteralTokenData::I64(i) => IntegerLikeLiteralTokenData::I64(-i),
            IntegerLikeLiteralTokenData::I128(i) => IntegerLikeLiteralTokenData::I128(-i),
            IntegerLikeLiteralTokenData::ISize(i) => IntegerLikeLiteralTokenData::ISize(-i),
            IntegerLikeLiteralTokenData::R8(_)
            | IntegerLikeLiteralTokenData::R16(_)
            | IntegerLikeLiteralTokenData::R32(_)
            | IntegerLikeLiteralTokenData::R64(_)
            | IntegerLikeLiteralTokenData::R128(_)
            | IntegerLikeLiteralTokenData::RSize(_)
            | IntegerLikeLiteralTokenData::U8(_)
            | IntegerLikeLiteralTokenData::U16(_)
            | IntegerLikeLiteralTokenData::U32(_)
            | IntegerLikeLiteralTokenData::U64(_)
            | IntegerLikeLiteralTokenData::U128(_)
            | IntegerLikeLiteralTokenData::USize(_) => return None,
        })
    }
}
