use super::*;

#[salsa::interned]
pub struct DecTraitConstraint {
    ty: DecTerm,
    trai: DecTerm,
}

impl DecTraitConstraint {
    pub(crate) fn display_fmt_with_db_and_ctx(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &::salsa::Db,
        _ctx: &DecSvarNameMap,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl DecTermRewriteCopy for DecTraitConstraint {
    fn substitute_copy(self, _db: &::salsa::Db, _substitution: &DecTermSubstitution) -> Self {
        todo!()
    }
}
