use super::*;

#[salsa::interned(db = EtherealTermDb, jar = EtherealTermJar)]
pub struct EtherealTermTraitConstraint {
    ty: EtherealTerm,
    trai: EtherealTerm,
}

#[test]
fn term_trait_constraint_size_works() {
    assert_eq!(
        std::mem::size_of::<EtherealTermTraitConstraint>(),
        std::mem::size_of::<u32>()
    );
}

impl EtherealTermTraitConstraint {
    pub(crate) fn from_raw_unchecked(
        db: &dyn EtherealTermDb,
        valid_term: RawTermTraitConstraint,
        term_ty_expectation: TermTypeExpectation,
    ) -> TermResult<Self> {
        todo!()
    }

    pub(super) fn check(self, db: &dyn EtherealTermDb) -> TermResult<()> {
        todo!()
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EtherealTermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl EtherealTermTraitConstraint {
    fn substitute(self, db: &dyn EtherealTermDb, substituation: &TermSubstitution) -> EtherealTerm {
        todo!()
    }
}
