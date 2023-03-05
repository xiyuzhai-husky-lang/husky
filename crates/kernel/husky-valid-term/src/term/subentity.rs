use super::*;
use husky_word::Identifier;

#[salsa::interned(db = ValidTermDb, jar = ValidTermJar)]
pub struct ValidTermSubentity {
    parent: ValidTerm,
    ident: Identifier,
}

impl ValidTermSubentity {
    pub fn from_precise(db: &dyn PreciseTermDb, precise_term: PreciseTermSubentity) -> Self {
        todo!()
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn ValidTermDb,
        ctx: &mut ValidTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl ValidTermRewriteCopy for ValidTermSubentity {
    fn substitute_copy(self, db: &dyn ValidTermDb, substituation: &ValidTermSubstitution) -> Self {
        let old_parent = self.parent(db);
        let parent = old_parent.substitute_copy(db, substituation);
        if old_parent == parent {
            return self;
        }
        let ident = self.ident(db);
        ValidTermSubentity::new(db, parent, ident)
    }
}
