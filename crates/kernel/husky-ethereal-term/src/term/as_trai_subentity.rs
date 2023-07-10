use super::*;
use husky_coword::Ident;

#[salsa::interned(db = EtherealTermDb, jar = EtherealTermJar)]
pub struct EtherealTermAsTraitSubentity {
    parent: EtherealTerm,
    trai: EtherealTerm,
    ident: Ident,
}

#[test]
fn term_as_trai_subentity_size_works() {
    assert_eq!(
        std::mem::size_of::<EtherealTermAsTraitSubentity>(),
        std::mem::size_of::<u32>()
    );
}

impl EtherealTermAsTraitSubentity {
    pub(crate) fn from_declarative(
        db: &dyn EtherealTermDb,
        valid_term: DeclarativeTermAsTraitSubentity,
        term_ty_expectation: TermTypeExpectation,
    ) -> EtherealTermResult<Self> {
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

impl EtherealTermAsTraitSubentity {
    fn substitute(self, db: &dyn EtherealTermDb, substituation: &TermSubstitution) -> EtherealTerm
    where
        Self: Copy,
    {
        todo!()
        // let old_parent = self.parent(db);
        // let parent = old_parent.substitute(db, substituation);
        // let old_trai = self.trai(db);
        // let trai = old_trai.substitute(db, substituation);
        // if old_parent == parent && old_trai == trai {
        //     return self;
        // }
        // let ident = self.ident(db);
        // EtherealTermAsTraitSubentity::new(db, parent, trai, ident)
    }
}
