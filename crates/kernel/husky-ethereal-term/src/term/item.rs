use super::*;
use husky_coword::Ident;

#[salsa::interned(db = EtherealTermDb, jar = EtherealTermJar)]
pub struct EtherealTermSubitem {
    parent: EtherealTerm,
    ident: Ident,
}

#[test]
fn term_subitem_size_works() {
    assert_eq!(
        std::mem::size_of::<EtherealTermSubitem>(),
        std::mem::size_of::<u32>()
    );
}

impl EtherealTermSubitem {
    #[inline(always)]
    pub(crate) fn from_declarative(
        db: &dyn EtherealTermDb,
        valid_term: DeclarativeTermSubitem,
        term_ty_expectation: TermTypeExpectation,
    ) -> EtherealTermResult<EtherealTerm> {
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

    pub fn substitute(
        self,
        db: &dyn EtherealTermDb,
        substituation: &TermSubstitution,
    ) -> EtherealTerm {
        todo!()
        // let old_parent = self.parent(db);
        // let parent = old_parent.substitute(db, substituation);
        // if old_parent == parent {
        //     return self.into();
        // }
        // let ident = self.ident(db);
        // EtherealTermSubitem::new(db, parent, ident)
    }
}
