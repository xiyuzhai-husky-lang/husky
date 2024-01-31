use crate::*;

#[salsa::interned(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
pub struct AbstractionDeclarativeTerm {
    pub x: RuneDeclarativeTerm,
    pub m: DeclarativeTerm,
}

impl AbstractionDeclarativeTerm {
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

impl DeclarativeTermRewriteCopy for AbstractionDeclarativeTerm {
    fn substitute_copy(
        self,
        _db: &::salsa::Db,
        _substituation: &DeclarativeTermSubstitution,
    ) -> Self {
        todo!()
    }
}

impl salsa::DisplayWithDb for AbstractionDeclarativeTerm {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}
