use super::*;
use husky_coword::Ident;
use husky_entity_path::path::assoc_item::trai_item::TraitItemPath;

#[salsa::interned]
pub struct EthTypeAsTraitItem {
    pub parent: EthTerm,
    pub trai: EthTerm,
    pub ident: Ident,
    pub trai_item_path: TraitItemPath,
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
    pub(crate) fn display_fmt_with_db(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &::salsa::Db,
    ) -> std::fmt::Result {
        todo!()
    }
}

/// # reduce

impl EthTypeAsTraitItem {
    pub(in crate::term) fn reduce(self, db: &::salsa::Db) -> EthTerm {
        reduce_eth_ty_as_trai_item(db, self)
    }
}

#[salsa::tracked]
fn reduce_eth_ty_as_trai_item(db: &::salsa::Db, term: EthTypeAsTraitItem) -> EthTerm {
    match term.parent(db) {
        EthTerm::Literal(_) => todo!(),
        EthTerm::SymbolicVariable(_) => term.into(),
        EthTerm::LambdaVariable(_) => todo!(),
        EthTerm::ItemPath(_) => todo!(),
        EthTerm::Category(_) => todo!(),
        EthTerm::Universe(_) => todo!(),
        EthTerm::Curry(_) => todo!(),
        EthTerm::Ritchie(_) => todo!(),
        EthTerm::Abstraction(_) => todo!(),
        EthTerm::Application(_) => todo!(),
        EthTerm::TypeAsTraitItem(_) => todo!(),
        EthTerm::TraitConstraint(_) => todo!(),
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
    type Output = EthTerm;

    fn instantiate(
        self,
        instantiation: &EthInstantiation,
        ctx: &impl IsEthInstantiationContext,
        db: &::salsa::Db,
    ) -> Self::Output {
        let parent = self.parent(db).instantiate(instantiation, ctx, db);
        let trai = self.trai(db).instantiate(instantiation, ctx, db);
        let ident = self.ident(db);
        let trai_item_path = self.trai_item_path(db);
        let slf = Self::new(db, parent, trai, ident, trai_item_path);
        ctx.reduce_ty_as_trai_item(slf)
    }
}
