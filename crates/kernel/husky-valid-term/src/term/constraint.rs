use super::*;

#[salsa::interned(db = RawTermDb, jar = RawTermJar)]
pub struct RawTermTraitConstraint {
    ty: RawTerm,
    trai: RawTerm,
}

impl RawTermTraitConstraint {
    pub fn from_precise(db: &dyn RawTermDb, precise_term: RawTermTraitConstraint) -> Self {
        todo!()
    }

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
    fn substitute(self, db: &dyn RawTermDb, substituation: &RawTermSubstitution) -> Self {
        todo!()
    }
}
