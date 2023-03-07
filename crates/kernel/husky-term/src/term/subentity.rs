use super::*;
use husky_word::Identifier;

#[salsa::interned(db = TermDb, jar = TermJar)]
pub struct TermSubentity {
    parent: Term,
    ident: Identifier,
}

impl TermSubentity {
    #[inline(always)]
    pub(crate) fn from_raw_unchecked(
        db: &dyn TermDb,
        valid_term: RawTermSubentity,
        term_ty_expectation: TermTypeExpectation,
    ) -> TermResult<Term> {
        todo!()
    }

    pub(super) fn check(self, db: &dyn TermDb) -> TermResult<()> {
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

    pub fn substitute(self, db: &dyn TermDb, substituation: &TermSubstitution) -> Term {
        todo!()
        // let old_parent = self.parent(db);
        // let parent = old_parent.substitute(db, substituation);
        // if old_parent == parent {
        //     return self.into();
        // }
        // let ident = self.ident(db);
        // TermSubentity::new(db, parent, ident)
    }
}

#[salsa::tracked(jar = TermJar)]
pub(crate) fn check_term_subentity_validity(
    db: &dyn TermDb,
    term_subentity: TermSubentity,
) -> TermResult<()> {
    todo!()
}
