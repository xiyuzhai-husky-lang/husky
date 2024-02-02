use crate::*;

#[salsa::interned(db = DecTermDb, jar = DecTermJar)]
pub struct TraitConstraintDecTerm {
    ty: DecTerm,
    trai: DecTerm,
}

impl TraitConstraintDecTerm {
    pub(crate) fn display_fmt_with_db_and_ctx(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &::salsa::Db,
        _ctx: &mut DecTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl DecTermRewriteCopy for TraitConstraintDecTerm {
    fn substitute_copy(self, _db: &::salsa::Db, _substitution: &DecTermSubstitution) -> Self {
        todo!()
    }
}
