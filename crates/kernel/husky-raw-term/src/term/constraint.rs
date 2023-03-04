use crate::*;

#[salsa::interned(db = RawTermDb, jar = RawTermJar)]
pub struct RawTermTraitConstraint {
    ty: RawTerm,
    trai: RawTerm,
}

impl RawTermTraitConstraint {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn RawTermDb,
        ctx: &mut RawTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl RawTermRewriteCopy for RawTermTraitConstraint {
    fn substitute_copy(self, db: &dyn RawTermDb, substituation: &RawTermSubstitution) -> Self {
        todo!()
    }
}
