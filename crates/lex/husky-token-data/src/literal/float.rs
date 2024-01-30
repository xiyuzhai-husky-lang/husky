use super::*;
use husky_term_prelude::literal::float::{TermF32Literal, TermF64Literal};

#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum FloatLiteralData {
    Unspecified(UnspecifiedFloatLiteral),
    F32(TermF32Literal),
    F64(TermF64Literal),
}

#[salsa::tracked(db = TokenDataDb, jar = TokenDataJar)]
pub struct UnspecifiedFloatLiteral {
    #[return_ref]
    pub text: String,
}

impl FloatLiteralData {
    pub fn negative(self, db: &::salsa::Db) -> Self {
        match self {
            FloatLiteralData::Unspecified(slf) => {
                let mut text = "-".to_string();
                text += slf.text(db);
                UnspecifiedFloatLiteral::new(db, text).into()
            }
            FloatLiteralData::F32(slf) => {
                let mut text = "-".to_string();
                text += slf.text(db);
                TermF32Literal::new(db, -slf.value(db), text).into()
            }
            FloatLiteralData::F64(slf) => {
                let mut text = "-".to_string();
                text += slf.text(db);
                TermF64Literal::new(db, -slf.value(db), text).into()
            }
        }
    }
}
