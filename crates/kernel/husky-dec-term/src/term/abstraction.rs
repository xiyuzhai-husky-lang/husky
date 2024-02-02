use crate::*;

#[salsa::interned(db = DecTermDb, jar = DecTermJar)]
pub struct AbstractionDecTerm {
    pub x: RuneDecTerm,
    pub m: DecTerm,
}

impl AbstractionDecTerm {
    pub fn ty(&self) -> DecTerm {
        todo!()
    }

    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &::salsa::Db,
        _ctx: &mut DecTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl DecTermRewriteCopy for AbstractionDecTerm {
    fn substitute_copy(self, _db: &::salsa::Db, substitution: &DecTermSubstitution) -> Self {
        todo!()
    }
}

impl salsa::DisplayWithDb for AbstractionDecTerm {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}
