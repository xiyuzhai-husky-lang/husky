use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirConstant {
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
    // we don't use usize here due to cross-compilation
    USize(usize),
    R8(u8),
    R16(u16),
    R32(u32),
    R64(u64),
    R128(u128),
    RSize(usize),
    // todo: should we add more types here?
    // Rust only allows for these things
    Symbol(HirConstSymbol),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SymbolIndex {
    ty_family: HirTypeFamily,
    index: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum HirTypeFamily {
    PathLeading(TypePath),
    Symbol,
}
