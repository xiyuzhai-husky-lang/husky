use super::*;
use husky_coword::Ident;

#[salsa::interned(db = EthTermDb, jar = EthTermJar)]
pub struct EthTypeAsTraitItem {
    parent: EthTerm,
    trai: EthTerm,
    ident: Ident,
}

#[test]
fn term_as_trai_subitem_size_works() {
    assert_eq!(
        std::mem::size_of::<EthTypeAsTraitItem>(),
        std::mem::size_of::<u32>()
    );
}

impl EthTypeAsTraitItem {
    pub(crate) fn from_dec(
        _db: &::salsa::Db,
        _valid_term: DecTypeAsTraitItem,
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

impl EthTypeAsTraitItem {
    pub fn substitute(self, _substitution: EthTermSubstitution, _db: &::salsa::Db) -> EthTerm
    where
        Self: Copy,
    {
        todo!()
        // let old_parent = self.parent(db);
        // let parent = old_parent.substitute(substitution, db, );
        // let old_trai = self.trai(db);
        // let trai = old_trai.substitute(substitution, db, );
        // if old_parent == parent && old_trai == trai {
        //     return self;
        // }
        // let ident = self.ident(db);
        // EthTermAsTraitSubitem::new(db, parent, trai, ident)
    }
}

impl EthInstantiate for EthTypeAsTraitItem {
    type Output = Self;

    fn instantiate(self, db: &salsa::Db, instantiation: &EthInstantiation) -> Self::Output {
        Self::new(
            db,
            self.parent(db).instantiate(db, instantiation),
            self.trai(db).instantiate(db, instantiation),
            self.ident(db),
        )
    }
}
