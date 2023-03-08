use crate::*;

#[salsa::interned(db = RawTermDb, jar = RawTermJar)]
pub struct RawTermTraitConstraint {
    ty: RawTerm,
    trai: RawTerm,
}

impl RawTermTraitConstraint {
    pub(crate) fn show_with_db_fmt(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &dyn RawTermDb,
        _ctx: &mut RawTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl RawTermRewriteCopy for RawTermTraitConstraint {
    fn substitute(self, _db: &dyn RawTermDb, _substituation: &RawTermSubstitution) -> Self {
        todo!()
    }
}
