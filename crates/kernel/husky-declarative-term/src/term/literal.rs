use salsa::Database;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
#[enum_class::from_variants]
pub enum DeclarativeTermLiteral {
    Resolved(TermLiteral),
    Unresolved(UnresolvedTermLiteral),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
pub enum UnresolvedTermLiteral {
    RegularInteger(i128),
}

impl DeclarativeTermLiteral {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        _db: &Db,
        _ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        f.write_str("DeclarativeTermLiteralTodo")
    }
}

impl salsa::DisplayWithDb for DeclarativeTermLiteral {
    fn display_with_db_fmt(&self, f: &mut std::fmt::Formatter<'_>, db: &Db) -> std::fmt::Result {
        let db = db();
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl std::fmt::Display for DeclarativeTermLiteral {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
