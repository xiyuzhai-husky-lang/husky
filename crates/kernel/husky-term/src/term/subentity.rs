use super::*;
use husky_word::Identifier;

#[salsa::interned(db = TermDb, jar = TermJar)]
pub struct TermSubentity {
    parent: Term,
    ident: Identifier,
}

impl TermSubentity {
    #[inline(always)]
    pub fn from_valid(db: &dyn ValidTermDb, valid_term: ValidTermSubentity) -> Self {
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

impl TermRewriteCopy for TermSubentity {
    fn substitute_copy(self, db: &dyn TermDb, substituation: &TermSubstitution) -> Self {
        let old_parent = self.parent(db);
        let parent = old_parent.substitute_copy(db, substituation);
        if old_parent == parent {
            return self;
        }
        let ident = self.ident(db);
        TermSubentity::new(db, parent, ident)
    }
}
