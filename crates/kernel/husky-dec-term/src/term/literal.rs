use crate::*;
use husky_term_prelude::literal::{
    int::{
        TermI128Literal, TermI64Literal, TermISizeLiteral, TermR128Literal, TermR64Literal,
        TermRSizeLiteral, TermU128Literal, TermU64Literal, TermUSizeLiteral,
    },
    Literal,
};
use husky_token_data::{IntegerLikeLiteralTokenData, LiteralTokenData};

use self::name::SymbolDecTermNameMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum DecLiteral {
    Resolved(Literal),
    Unresolved(UnresolvedDecLiteral),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
pub enum UnresolvedDecLiteral {
    RegularInteger(i128),
}

impl DecLiteral {
    pub(crate) fn display_fmt_with_db_and_ctx(
        self,
        f: &mut std::fmt::Formatter<'_>,
        _db: &::salsa::Db,
        _ctx: &SymbolDecTermNameMap,
    ) -> std::fmt::Result {
        f.write_str("DecTermLiteralTodo")
    }
}

impl DecLiteral {
    pub fn from_literal_token_data(literal: LiteralTokenData, db: &salsa::Db) -> Self {
        match literal {
            LiteralTokenData::Unit => Literal::Unit(()).into(),
            LiteralTokenData::Char(_) => todo!(),
            LiteralTokenData::String(_) => todo!(),
            LiteralTokenData::Integer(literal) => match literal {
                IntegerLikeLiteralTokenData::UnspecifiedRegular(i) => {
                    UnresolvedDecLiteral::RegularInteger(i).into()
                }
                IntegerLikeLiteralTokenData::UnspecifiedLarge() => todo!(),
                IntegerLikeLiteralTokenData::I8(i) => Literal::I8(i).into(),
                IntegerLikeLiteralTokenData::I16(i) => Literal::I16(i).into(),
                IntegerLikeLiteralTokenData::I32(i) => Literal::I32(i).into(),
                IntegerLikeLiteralTokenData::I64(i) => {
                    Literal::I64(TermI64Literal::new(db, i)).into()
                }
                IntegerLikeLiteralTokenData::I128(r) => {
                    Literal::I128(TermI128Literal::new(db, r)).into()
                }
                IntegerLikeLiteralTokenData::ISize(i) => {
                    Literal::ISize(TermISizeLiteral::new(db, i as i64)).into()
                }
                IntegerLikeLiteralTokenData::R8(r) => Literal::R8(r).into(),
                IntegerLikeLiteralTokenData::R16(r) => Literal::R16(r).into(),
                IntegerLikeLiteralTokenData::R32(r) => Literal::R32(r).into(),
                IntegerLikeLiteralTokenData::R64(r) => {
                    Literal::R64(TermR64Literal::new(db, r)).into()
                }
                IntegerLikeLiteralTokenData::R128(r) => {
                    Literal::R128(TermR128Literal::new(db, r)).into()
                }
                IntegerLikeLiteralTokenData::RSize(r) => {
                    Literal::RSize(TermRSizeLiteral::new(db, r as u64)).into()
                }
                IntegerLikeLiteralTokenData::U8(u) => Literal::U8(u).into(),
                IntegerLikeLiteralTokenData::U16(u) => Literal::U16(u).into(),
                IntegerLikeLiteralTokenData::U32(u) => Literal::U32(u).into(),
                IntegerLikeLiteralTokenData::U64(u) => {
                    Literal::U64(TermU64Literal::new(db, u)).into()
                }
                IntegerLikeLiteralTokenData::U128(u) => {
                    Literal::U128(TermU128Literal::new(db, u)).into()
                }
                IntegerLikeLiteralTokenData::USize(u) => {
                    Literal::USize(TermUSizeLiteral::new(db, u as u64)).into()
                }
            },
            LiteralTokenData::Float(_) => todo!(),
            LiteralTokenData::Bool(_) => todo!(),
        }
    }
}

impl DecTerm {
    pub fn from_literal_token_data(literal: LiteralTokenData, db: &salsa::Db) -> Self {
        DecLiteral::from_literal_token_data(literal, db).into()
    }
}
