use super::*;

#[salsa::interned(db = TermDb, jar = TermJar)]
pub struct TermTraitConstraint {
    ty: Term,
    trai: Term,
}

#[test]
fn term_trait_constraint_size_works() {
    assert_eq!(
        std::mem::size_of::<TermTraitConstraint>(),
        std::mem::size_of::<u32>()
    );
}

impl TermTraitConstraint {
    pub(crate) fn from_raw_unchecked(
        db: &dyn TermDb,
        valid_term: RawTermTraitConstraint,
        term_ty_expectation: TermTypeExpectation,
    ) -> TermResult<Self> {
        todo!()
    }

    pub(super) fn check(self, db: &dyn TermDb) -> TermResult<()> {
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

#[salsa::tracked(jar = TermJar)]
pub(crate) fn check_term_trai_constraint_validity(
    db: &dyn TermDb,
    term_trai_constraint: TermTraitConstraint,
) -> TermResult<()> {
    todo!()
}

impl TermTraitConstraint {
    fn substitute(self, db: &dyn TermDb, substituation: &TermSubstitution) -> Term {
        todo!()
    }
}
