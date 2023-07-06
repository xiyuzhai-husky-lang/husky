use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
#[enum_class::from_variants]
pub enum DeclarativeTermLiteral {
    Resolved(TermLiteral),
    Unresolved(UnresolvedTermLiteral),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
pub enum UnresolvedTermLiteral {
    Integer64(i64),
}

impl UnresolvedTermLiteral {
    pub fn new_unresolved_integer(s: &str) -> Self {
        todo!()
    }
}

impl DeclarativeTermLiteral {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        _db: &dyn DeclarativeTermDb,
        _ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        f.write_str("DeclarativeTermLiteralTodo")
    }
}

impl<Db: DeclarativeTermDb + ?Sized> salsa::DisplayWithDb<Db> for DeclarativeTermLiteral {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        _level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<DeclarativeTermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl std::fmt::Display for DeclarativeTermLiteral {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
