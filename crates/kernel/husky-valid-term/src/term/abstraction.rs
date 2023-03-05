use super::*;

#[salsa::interned(db = ValidTermDb, jar = ValidTermJar, constructor = new_inner)]
pub struct ValidTermAbstraction {
    x: ValidTermSymbol,
    m: ValidTerm,
}

impl ValidTermAbstraction {
    pub fn from_precise(db: &dyn PreciseTermDb, precise_term: PreciseTermAbstraction) -> Self {
        todo!()
    }

    pub fn ty(&self) -> ValidTerm {
        todo!()
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn ValidTermDb,
        ctx: &mut ValidTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl ValidTermRewriteCopy for ValidTermAbstraction {
    fn substitute_copy(self, db: &dyn ValidTermDb, substituation: &ValidTermSubstitution) -> Self {
        todo!()
    }
}

impl<Db: ValidTermDb + ?Sized> salsa::DisplayWithDb<Db> for ValidTermAbstraction {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        // use std::fmt::Write;
        // f.write_char(husky_unicode_symbols::greek::GREEK_LETTER_LOWERCASE_LAMBDA);
        // todo!()
        let db = <Db as salsa::DbWithJar<ValidTermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}
