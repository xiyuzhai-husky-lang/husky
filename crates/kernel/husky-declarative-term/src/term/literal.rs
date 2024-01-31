use crate::*;
use husky_term_prelude::literal::TermLiteral;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum DeclarativeTermLiteral {
    Resolved(TermLiteral),
    Unresolved(UnresolvedTermLiteral),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
pub enum UnresolvedTermLiteral {
    RegularInteger(i128),
}

impl DeclarativeTermLiteral {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        _db: &::salsa::Db,
        _ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        f.write_str("DeclarativeTermLiteralTodo")
    }
}

impl salsa::DisplayWithDb for DeclarativeTermLiteral {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}
