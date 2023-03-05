use super::*;

#[salsa::interned(db = PreciseTermDb, jar = PreciseTermJar)]
pub struct PreciseTermTraitConstraint {
    ty: PreciseTerm,
    trai: PreciseTerm,
}

impl PreciseTermTraitConstraint {
    pub fn from_raw(
        db: &dyn PreciseTermDb,
        raw_term: RawTermTraitConstraint,
        raw_ty_expectation: TypeExpectation,
    ) -> PreciseTermResult<Self> {
        todo!()
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn PreciseTermDb,
        ctx: &mut PreciseTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl PreciseTermRewriteCopy for PreciseTermTraitConstraint {
    fn substitute_copy(
        self,
        db: &dyn PreciseTermDb,
        substituation: &PreciseTermSubstitution,
    ) -> Self {
        todo!()
    }
}
