use super::*;
use husky_coword::Ident;

#[salsa::interned(db = EtherealTermDb, jar = EtherealTermJar)]
pub struct TypeAsTraitItemEtherealTerm {
    parent: EtherealTerm,
    trai: EtherealTerm,
    ident: Ident,
}

#[test]
fn term_as_trai_subitem_size_works() {
    assert_eq!(
        std::mem::size_of::<TypeAsTraitItemEtherealTerm>(),
        std::mem::size_of::<u32>()
    );
}

impl TypeAsTraitItemEtherealTerm {
    pub(crate) fn from_declarative(
        _db: &::salsa::Db,
        _valid_term: TypeAsTraitItemDeclarativeTerm,
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

impl TypeAsTraitItemEtherealTerm {
    pub fn substitute(
        self,
        substitution: EtherealTermSubstitution,
        _db: &::salsa::Db,
    ) -> EtherealTerm
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
        // EtherealTermAsTraitSubitem::new(db, parent, trai, ident)
    }
}
