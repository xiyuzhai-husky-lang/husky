use super::*;

#[salsa::interned(db = DecTermDb, jar = DecTermJar)]
pub struct DecAbstraction {
    pub x: DecHvar,
    pub m: DecTerm,
}

impl DecAbstraction {
    pub fn ty(&self) -> DecTerm {
        todo!()
    }

    #[inline(never)]
    pub(crate) fn display_fmt_with_db_and_ctx(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &::salsa::Db,
        _ctx: &DecSvarNameMap,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl DecTermRewriteCopy for DecAbstraction {
    fn substitute_copy(self, _db: &::salsa::Db, _substitution: &DecTermSubstitution) -> Self {
        todo!()
    }
}
