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
        _db: &::salsa::Db,
        _ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl DeclarativeTermRewriteCopy for DeclarativeTermAbstraction {
    fn substitute_copy(
        self,
        _db: &::salsa::Db,
        _substituation: &DeclarativeTermSubstitution,
    ) -> Self {
        todo!()
    }
}

impl salsa::DisplayWithDb for DeclarativeTermAbstraction {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        // use std::fmt::Write;
        // f.write_char(husky_unicode_symbols::greek::GREEK_LETTER_LOWERCASE_LAMBDA);
        // todo!()
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}
