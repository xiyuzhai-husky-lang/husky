use std::sync::Arc;

use crate::*;
use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LiteralToken {
    Unit,
    Integer(i64),
    I32(i32),
    I64(i64),
    Float(OrderedFloat<f64>),
    F32(OrderedFloat<f32>),
    F64(OrderedFloat<f64>),
    Bits(u64),
    B32(u32),
    B64(u64),
    Bool(bool),
    String(StringLiteral),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct StringLiteral {
    data: String,
}

impl StringLiteral {
    pub fn new(data: String) -> Self {
        Self { data }
    }
}

impl LiteralToken {
    pub fn negative(self) -> Option<LiteralToken> {
        match self {
            LiteralToken::Unit => None,
            LiteralToken::Integer(i) => Some(LiteralToken::Integer(-i)),
            LiteralToken::I32(i) => Some(LiteralToken::I32(-i)),
            LiteralToken::I64(i) => Some(LiteralToken::I64(-i)),
            LiteralToken::Float(f) => Some(LiteralToken::Float(-f)),
            LiteralToken::F32(f) => Some(LiteralToken::F32(-f)),
            LiteralToken::F64(f) => Some(LiteralToken::F64(-f)),
            LiteralToken::Bits(_) => None,
            LiteralToken::B32(_) => None,
            LiteralToken::B64(_) => None,
            LiteralToken::Bool(_) => None,
            LiteralToken::String(_) => None,
        }
    }
}
