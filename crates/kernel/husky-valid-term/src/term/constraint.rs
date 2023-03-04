use crate::*;

#[salsa::interned(db = ValidTermDb, jar = ValidTermJar)]
pub struct ValidTermTraitConstraint {
    ty: ValidTerm,
    trai: ValidTerm,
}

impl ValidTermTraitConstraint {
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
    fn substitute_copy(self, db: &dyn ValidTermDb, substituation: &ValidTermSubstitution) -> Self {
        todo!()
    }
}
