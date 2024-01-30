use super::*;
use husky_term_prelude::literal::float::{TermF32Literal, TermF64Literal};

use std::ops::Neg;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum FloatLiteralData {
    Unspecified(UnspecifiedFloatLiteral),
    F32(TermF32Literal),
    F64(TermF64Literal),
}

#[salsa::tracked(db = TokenDataDb, jar = TokenDataJar)]
pub struct UnspecifiedFloatLiteral {
    #[return_ref]
    pub data: String,
}

impl FloatLiteralData {
    pub fn negative(self, db: &::salsa::Db) -> Self {
        match self {
            FloatLiteralData::Unspecified(slf) => todo!(),
            FloatLiteralData::F32(slf) => todo!(),
            FloatLiteralData::F64(slf) => todo!(),
        }
    }
}
