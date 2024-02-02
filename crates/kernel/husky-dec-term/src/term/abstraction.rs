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
    pub(crate) fn display_fmt_with_db_and_ctx(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &::salsa::Db,
        _ctx: &mut DecTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl DecTermRewriteCopy for AbstractionDecTerm {
    fn substitute_copy(self, _db: &::salsa::Db, _substitution: &DecTermSubstitution) -> Self {
        todo!()
    }
}
