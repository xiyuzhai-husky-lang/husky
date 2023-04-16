use super::*;
use husky_word::Ident;

#[salsa::interned(db = TermDb, jar = TermJar)]
pub struct TermAsTraitSubentity {
    parent: Term,
    trai: Term,
    ident: Ident,
}

#[test]
fn term_as_trai_subentity_size_works() {
    assert_eq!(
        std::mem::size_of::<TermAsTraitSubentity>(),
        std::mem::size_of::<u32>()
    );
}

impl TermAsTraitSubentity {
    pub(crate) fn from_raw_unchecked(
        db: &dyn TermDb,
        valid_term: RawTermAsTraitSubentity,
        term_ty_expectation: TermTypeExpectation,
    ) -> TermResult<Self> {
        todo!()
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn TermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl TermAsTraitSubentity {
    fn substitute(self, db: &dyn TermDb, substituation: &TermSubstitution) -> Term
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
        // TermAsTraitSubentity::new(db, parent, trai, ident)
    }
}
