use crate::*;
use husky_token::StringLiteral;
use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = TermPreludeDb, jar = TermPreludeJar)]
pub enum PreciseTermLiteral {
    Unit,
    I32(i32),
    I64(i64),
    Nat(PreciseTermNaturalNumber),
    Float(OrderedFloat<f64>),
    F32(OrderedFloat<f32>),
    F64(OrderedFloat<f64>),
    B8(u8),
    B16(u16),
    B32(u32),
    B64(u64),
    Bool(bool),
    Str(StringLiteral),
    EvalLifetime,
    StaticLifetime,
}

/// allowing representing very large number
#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct PreciseTermInteger128 {
    pub value: i128,
}

/// allowing representing very large number
#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct PreciseTermInteger256 {
    pub value: [i128; 2],
}

/// allowing representing very large number
#[salsa::interned(db = TermPreludeDb, jar = TermPreludeJar)]
pub struct PreciseTermNaturalNumber {
    pub bits: Vec<usize>,
}
