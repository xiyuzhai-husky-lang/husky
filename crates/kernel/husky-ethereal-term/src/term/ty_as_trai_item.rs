use super::*;
use husky_coword::Ident;

#[salsa::interned(db = EthTermDb, jar = EthTermJar)]
pub struct TypeAsTraitItemEthTerm {
    parent: EthTerm,
    trai: EthTerm,
    ident: Ident,
}

#[test]
fn term_as_trai_subitem_size_works() {
    assert_eq!(
        std::mem::size_of::<TypeAsTraitItemEthTerm>(),
        std::mem::size_of::<u32>()
    );
}

impl TypeAsTraitItemEthTerm {
    pub(crate) fn from_declarative(
        _db: &::salsa::Db,
        _valid_term: TypeAsTraitItemDeclarativeTerm,
        _term_ty_expectation: TermTypeExpectation,
    ) -> EthTermResult<Self> {
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

/// # rewrite

impl TypeAsTraitItemEthTerm {
    pub fn substitute(self, substitution: EthTermSubstitution, _db: &::salsa::Db) -> EthTerm
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

impl EthTermInstantiate for TypeAsTraitItemEthTerm {
    type Output = Self;

    fn instantiate(self, db: &salsa::Db, instantiation: &EtherealInstantiation) -> Self::Output {
        Self::new(
            db,
            self.parent(db).instantiate(db, instantiation),
            self.trai(db).instantiate(db, instantiation),
            self.ident(db),
        )
    }
}
