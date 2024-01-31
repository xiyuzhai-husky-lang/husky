use super::*;

#[salsa::interned(db = EtherealTermDb, jar = EtherealTermJar)]
pub struct TraitConstraintEtherealTerm {
    ty: EtherealTerm,
    trai: EtherealTerm,
}

#[test]
fn term_trait_constraint_size_works() {
    assert_eq!(
        std::mem::size_of::<TraitConstraintEtherealTerm>(),
        std::mem::size_of::<u32>()
    );
}

impl TraitConstraintEtherealTerm {
    pub(crate) fn from_declarative(
        _db: &::salsa::Db,
        _valid_term: TraitConstraintDeclarativeTerm,
        _term_ty_expectation: TermTypeExpectation,
    ) -> EtherealTermResult<Self> {
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

impl TraitConstraintEtherealTerm {
    pub fn substitute(self, substitution: EtherealTermSubstitution, db: &::salsa::Db) -> Self {
        let old_ty = self.ty(db);
        let new_ty = old_ty.substitute(substitution, db);
        let old_trai = self.trai(db);
        let new_trai = old_trai.substitute(substitution, db);
        if old_ty == new_ty && old_trai == new_trai {
            return self;
        }
        Self::new(db, new_ty, new_trai)
    }
}
