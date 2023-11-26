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
    pub(crate) fn from_declarative(
        db: &::salsa::Db,
        valid_term: DeclarativeTermTraitConstraint,
        term_ty_expectation: TermTypeExpectation,
    ) -> EtherealTermResult<Self> {
        todo!()
    }

    pub(super) fn check(self, db: &::salsa::Db) -> EtherealTermResult<()> {
        todo!()
    }

    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl EtherealTermTraitConstraint {
    fn substitute(self, db: &::salsa::Db, substituation: &TermSubstitution) -> EtherealTerm {
        todo!()
    }
}
