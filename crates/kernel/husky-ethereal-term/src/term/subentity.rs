use super::*;
use husky_word::Ident;

#[salsa::interned(db = EtherealTermDb, jar = EtherealTermJar)]
pub struct EtherealTermSubentity {
    parent: EtherealTerm,
    ident: Ident,
}

#[test]
fn term_subentity_size_works() {
    assert_eq!(
        std::mem::size_of::<EtherealTermSubentity>(),
        std::mem::size_of::<u32>()
    );
}

impl EtherealTermSubentity {
    #[inline(always)]
    pub(crate) fn from_declarative(
        db: &dyn EtherealTermDb,
        valid_term: DeclarativeTermSubentity,
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
        // EtherealTermSubentity::new(db, parent, ident)
    }
}
