use super::*;
use husky_word::Identifier;

#[salsa::interned(db = TermDb, jar = TermJar)]
pub struct TermAsTraitSubentity {
    parent: Term,
    trai: Term,
    ident: Identifier,
}

impl TermAsTraitSubentity {
    pub(crate) fn from_raw_unchecked(
        db: &dyn TermDb,
        valid_term: RawTermAsTraitSubentity,
        term_ty_expectation: TermTypeExpectation,
    ) -> TermResult<Self> {
        todo!()
    }

    pub(super) fn check(self, db: &dyn TermDb) -> TermResult<()> {
        check_term_as_trai_subentity_validity(db, self)
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

#[salsa::tracked(jar = TermJar)]
pub(crate) fn check_term_as_trai_subentity_validity(
    db: &dyn TermDb,
    term_as_trai_subentity: TermAsTraitSubentity,
) -> TermResult<()> {
    todo!()
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
