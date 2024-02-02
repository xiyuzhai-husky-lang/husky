use crate::*;
use husky_term_prelude::literal::{
    int::{
        TermI128Literal, TermI64Literal, TermISizeLiteral, TermR128Literal, TermR64Literal,
        TermRSizeLiteral, TermU128Literal, TermU64Literal, TermUSizeLiteral,
    },
    TermLiteral,
};
use husky_token_data::{IntegerLikeLiteralTokenData, LiteralTokenData};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum LiteralDecTerm {
    Resolved(TermLiteral),
    Unresolved(UnresolvedTermLiteral),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
pub enum UnresolvedTermLiteral {
    RegularInteger(i128),
}

impl LiteralDecTerm {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        _db: &::salsa::Db,
        _ctx: &mut DecTermShowContext,
    ) -> std::fmt::Result {
        f.write_str("DecTermLiteralTodo")
    }
}

impl salsa::DisplayWithDb for LiteralDecTerm {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl LiteralDecTerm {
    pub fn from_literal_token_data(literal: LiteralTokenData, db: &salsa::Db) -> Self {
        match literal {
            LiteralTokenData::Unit => TermLiteral::Unit(()).into(),
            LiteralTokenData::Char(_) => todo!(),
            LiteralTokenData::String(_) => todo!(),
            LiteralTokenData::Integer(literal) => match literal {
                IntegerLikeLiteralTokenData::UnspecifiedRegular(i) => {
                    UnresolvedTermLiteral::RegularInteger(i).into()
                }
                IntegerLikeLiteralTokenData::UnspecifiedLarge() => todo!(),
                IntegerLikeLiteralTokenData::I8(i) => TermLiteral::I8(i).into(),
                IntegerLikeLiteralTokenData::I16(i) => TermLiteral::I16(i).into(),
                IntegerLikeLiteralTokenData::I32(i) => TermLiteral::I32(i).into(),
                IntegerLikeLiteralTokenData::I64(i) => {
                    TermLiteral::I64(TermI64Literal::new(db, i)).into()
                }
                IntegerLikeLiteralTokenData::I128(r) => {
                    TermLiteral::I128(TermI128Literal::new(db, r)).into()
                }
                IntegerLikeLiteralTokenData::ISize(i) => {
                    TermLiteral::ISize(TermISizeLiteral::new(db, i as i64)).into()
                }
                IntegerLikeLiteralTokenData::R8(r) => TermLiteral::R8(r).into(),
                IntegerLikeLiteralTokenData::R16(r) => TermLiteral::R16(r).into(),
                IntegerLikeLiteralTokenData::R32(r) => TermLiteral::R32(r).into(),
                IntegerLikeLiteralTokenData::R64(r) => {
                    TermLiteral::R64(TermR64Literal::new(db, r)).into()
                }
                IntegerLikeLiteralTokenData::R128(r) => {
                    TermLiteral::R128(TermR128Literal::new(db, r)).into()
                }
                IntegerLikeLiteralTokenData::RSize(r) => {
                    TermLiteral::RSize(TermRSizeLiteral::new(db, r as u64)).into()
                }
                IntegerLikeLiteralTokenData::U8(u) => TermLiteral::U8(u).into(),
                IntegerLikeLiteralTokenData::U16(u) => TermLiteral::U16(u).into(),
                IntegerLikeLiteralTokenData::U32(u) => TermLiteral::U32(u).into(),
                IntegerLikeLiteralTokenData::U64(u) => {
                    TermLiteral::U64(TermU64Literal::new(db, u)).into()
                }
                IntegerLikeLiteralTokenData::U128(u) => {
                    TermLiteral::U128(TermU128Literal::new(db, u)).into()
                }
                IntegerLikeLiteralTokenData::USize(u) => {
                    TermLiteral::USize(TermUSizeLiteral::new(db, u as u64)).into()
                }
            },
            LiteralTokenData::Float(_) => todo!(),
            LiteralTokenData::TupleIndex(_) => todo!(),
            LiteralTokenData::Bool(_) => todo!(),
        }
    }
}

impl DecTerm {
    pub fn from_literal_token_data(literal: LiteralTokenData, db: &salsa::Db) -> Self {
        LiteralDecTerm::from_literal_token_data(literal, db).into()
    }
}
