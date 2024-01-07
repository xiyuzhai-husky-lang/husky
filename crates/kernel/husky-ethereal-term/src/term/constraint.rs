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
        _db: &::salsa::Db,
        _valid_term: DeclarativeTermTraitConstraint,
        _term_ty_expectation: TermTypeExpectation,
    ) -> EtherealTermResult<Self> {
        todo!()
    }

    pub(super) fn check(self, _db: &::salsa::Db) -> EtherealTermResult<()> {
        todo!()
    }

    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &::salsa::Db,
        _ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl EtherealTermTraitConstraint {
    fn substitute(self, _db: &::salsa::Db, _substituation: &TermSubstitution) -> EtherealTerm {
        todo!()
    }
}
