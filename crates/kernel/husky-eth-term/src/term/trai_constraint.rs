use super::*;

#[salsa::interned]
pub struct EthTraitConstraint {
    pub ty: EthTerm,
    pub trai: EthTerm,
}

#[test]
fn term_trait_constraint_size_works() {
    assert_eq!(
        std::mem::size_of::<EthTraitConstraint>(),
        std::mem::size_of::<u32>()
    );
}

impl EthTraitConstraint {
    pub(crate) fn from_dec(
        _db: &::salsa::Db,
        _valid_term: DecTraitConstraint,
        _term_ty_expectation: TypeFinalDestinationExpectation,
    ) -> EthTermResult<Self> {
        todo!()
    }

    #[inline(never)]
    pub(crate) fn display_fmt_with_db(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &::salsa::Db,
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

impl EthInstantiate for EthTraitConstraint {
    type Output = Self;

    fn instantiate(
        self,
        instantiation: &EthInstantiation,
        ctx: impl IsEthTermContextRef,
        db: &::salsa::Db,
    ) -> Self::Output {
        Self::new(
            db,
            self.ty(db).instantiate(instantiation, ctx, db),
            self.trai(db).instantiate(instantiation, ctx, db),
        )
    }
}
