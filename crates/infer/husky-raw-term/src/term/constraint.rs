use crate::*;

#[salsa::interned(db = TermDb, jar = TermJar)]
pub struct TermTraitConstraint {
    ty: Term,
    trai: Term,
}

impl TermTraitConstraint {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn TermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl TermRewriteCopy for TermTraitConstraint {
    fn substitute_copy(self, db: &dyn TermDb, substituation: &TermSubstitution) -> Self {
        todo!()
    }
}
