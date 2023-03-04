use crate::*;

#[salsa::interned(db = RawTermDb, jar = RawTermJar)]
pub struct RawTermAbstraction {
    x: RawTermSymbol,
    m: RawTerm,
}

impl RawTermAbstraction {
    pub fn ty(&self) -> RawTerm {
        todo!()
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn RawTermDb,
        ctx: &mut RawTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl RawTermRewriteCopy for RawTermAbstraction {
    fn substitute_copy(self, db: &dyn RawTermDb, substituation: &RawTermSubstitution) -> Self {
        todo!()
    }
}

impl<Db: RawTermDb + ?Sized> salsa::DisplayWithDb<Db> for RawTermAbstraction {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        // use std::fmt::Write;
        // f.write_char(husky_unicode_symbols::greek::GREEK_LETTER_LOWERCASE_LAMBDA);
        // todo!()
        let db = <Db as salsa::DbWithJar<RawTermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}
