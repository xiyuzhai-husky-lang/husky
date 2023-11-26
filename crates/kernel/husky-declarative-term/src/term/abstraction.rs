use crate::*;

#[salsa::interned(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
pub struct DeclarativeTermAbstraction {
    pub x: DeclarativeTermRune,
    pub m: DeclarativeTerm,
}

impl DeclarativeTermAbstraction {
    pub fn ty(&self) -> DeclarativeTerm {
        todo!()
    }

    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &Db,
        _ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl DeclarativeTermRewriteCopy for DeclarativeTermAbstraction {
    fn substitute(
        self,
        _db: &dyn DeclarativeTermDb,
        _substituation: &DeclarativeTermSubstitution,
    ) -> Self {
        todo!()
    }
}

impl salsa::DisplayWithDb for DeclarativeTermAbstraction {
    fn display_with_db_fmt(&self, f: &mut std::fmt::Formatter<'_>, db: &Db) -> std::fmt::Result {
        // use std::fmt::Write;
        // f.write_char(husky_unicode_symbols::greek::GREEK_LETTER_LOWERCASE_LAMBDA);
        // todo!()
        let db = db.as_jar_db_dyn::<DeclarativeTermJar>();
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}
