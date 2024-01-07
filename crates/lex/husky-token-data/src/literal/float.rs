use super::*;
use husky_term_prelude::float::{TermF32Literal, TermF64Literal};

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

impl Neg for FloatLiteralData {
    type Output = Self;

    fn neg(self) -> Self::Output {
        todo!()
    }
}
