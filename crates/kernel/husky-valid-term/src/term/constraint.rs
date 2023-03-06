use super::*;

#[salsa::interned(db = ValidTermDb, jar = ValidTermJar)]
pub struct ValidTermTraitConstraint {
    ty: ValidTerm,
    trai: ValidTerm,
}

impl ValidTermTraitConstraint {
    pub fn from_precise(db: &dyn ValidTermDb, precise_term: PreciseTermTraitConstraint) -> Self {
        todo!()
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn ValidTermDb,
        ctx: &mut ValidTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl ValidTermRewriteCopy for ValidTermTraitConstraint {
    fn substitute(self, db: &dyn ValidTermDb, substituation: &ValidTermSubstitution) -> Self {
        todo!()
    }
}
