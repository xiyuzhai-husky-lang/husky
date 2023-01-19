use crate::*;
use husky_token::StringLiteral;
use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TermLiteral {
    Unit,
    I32(i32),
    I64(i64),
    Nat(TermNaturalNumber),
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
}

/// allowing representing very large number
#[salsa::interned(jar = TermJar)]
pub struct TermInteger128 {
    pub value: i128,
}

/// allowing representing very large number
#[salsa::interned(jar = TermJar)]
pub struct TermInteger256 {
    pub value: [i128; 2],
}

/// allowing representing very large number
#[salsa::interned(jar = TermJar)]
pub struct TermNaturalNumber {
    pub bits: Vec<usize>,
}

impl From<i32> for Term {
    fn from(value: i32) -> Self {
        Term::Literal(value.into())
    }
}

impl From<i64> for Term {
    fn from(value: i64) -> Self {
        Term::Literal(value.into())
    }
}

impl std::fmt::Display for TermLiteral {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<i32> for TermLiteral {
    fn from(value: i32) -> Self {
        TermLiteral::I32(value)
    }
}

impl From<i64> for TermLiteral {
    fn from(value: i64) -> Self {
        TermLiteral::I64(value)
    }
}
