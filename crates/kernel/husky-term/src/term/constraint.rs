use super::*;

#[salsa::interned(db = TermDb, jar = TermJar)]
pub struct TermTraitConstraint {
    ty: Term,
    trai: Term,
}

impl TermTraitConstraint {
    pub(crate) fn from_raw_unchecked(
        db: &dyn TermDb,
        valid_term: RawTermTraitConstraint,
        term_ty_expectation: TermTypeExpectation,
    ) -> TermResult<Self> {
        todo!()
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn TermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl TermTraitConstraint {
    fn substitute(self, db: &dyn TermDb, substituation: &TermSubstitution) -> Term {
        todo!()
    }
}
