use super::*;

#[salsa::interned(db = TermDb, jar = TermJar)]
pub struct TermAbstraction {
    x: TermVariable,
    m: Term,
}

#[test]
fn term_abstraction_size_works() {
    assert_eq!(
        std::mem::size_of::<TermAbstraction>(),
        std::mem::size_of::<u32>()
    );
}

impl TermAbstraction {
    pub(crate) fn from_raw_unchecked(
        db: &dyn TermDb,
        precise_term: RawTermAbstraction,
        term_ty_expectation: TermTypeExpectation,
    ) -> TermResult<Self> {
        todo!()
    }

    pub(super) fn check(self, db: &dyn TermDb) -> TermResult<()> {
        check_term_abstraction_validity(db, self)
    }

    pub fn ty(&self) -> Term {
        todo!()
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn TermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn check_term_abstraction_validity(
    db: &dyn TermDb,
    term_abstraction: TermAbstraction,
) -> TermResult<()> {
    todo!()
}

impl TermAbstraction {
    fn substitute(self, db: &dyn TermDb, substituation: &TermSubstitution) -> Term {
        todo!()
    }
}

impl<Db: TermDb + ?Sized> salsa::DisplayWithDb<Db> for TermAbstraction {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        // use std::fmt::Write;
        // f.write_char(husky_unicode_symbols::greek::GREEK_LETTER_LOWERCASE_LAMBDA);
        // todo!()
        let db = <Db as salsa::DbWithJar<TermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}
