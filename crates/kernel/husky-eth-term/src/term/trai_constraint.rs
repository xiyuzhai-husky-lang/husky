use super::*;

#[salsa::interned(db = EthTermDb, jar = EthTermJar)]
pub struct EthTraitConstraint {
    ty: EthTerm,
    trai: EthTerm,
}

#[test]
fn term_trait_constraint_size_works() {
    assert_eq!(
        std::mem::size_of::<EthTraitConstraint>(),
        std::mem::size_of::<u32>()
    );
}

impl EthTraitConstraint {
    pub(crate) fn from_declarative(
        _db: &::salsa::Db,
        _valid_term: DecTraitConstraint,
        _term_ty_expectation: TypeFinalDestinationExpectation,
    ) -> EthTermResult<Self> {
        todo!()
    }

    #[inline(never)]
    pub(crate) fn display_fmt_with_db_and_ctx(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &::salsa::Db,
        _ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

/// # rewrite

impl EthTraitConstraint {
    pub fn substitute(self, substitution: EthTermSubstitution, db: &::salsa::Db) -> Self {
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

impl EthTermInstantiate for EthTraitConstraint {
    type Output = Self;

    fn instantiate(self, db: &salsa::Db, instantiation: &EtherealInstantiation) -> Self::Output {
        Self::new(
            db,
            self.ty(db).instantiate(db, instantiation),
            self.trai(db).instantiate(db, instantiation),
        )
    }
}
