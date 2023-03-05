use super::*;
use husky_word::Identifier;

#[salsa::interned(db = PreciseTermDb, jar = PreciseTermJar)]
pub struct PreciseTermSubentity {
    parent: PreciseTerm,
    ident: Identifier,
}

impl PreciseTermSubentity {
    pub fn from_raw(
        db: &dyn PreciseTermDb,
        raw_term: RawTermSubentity,
        raw_ty_expectation: RawTypeExpectation,
    ) -> Self {
        todo!()
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn PreciseTermDb,
        ctx: &mut PreciseTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl PreciseTermRewriteCopy for PreciseTermSubentity {
    fn substitute_copy(
        self,
        db: &dyn PreciseTermDb,
        substituation: &PreciseTermSubstitution,
    ) -> Self {
        let old_parent = self.parent(db);
        let parent = old_parent.substitute_copy(db, substituation);
        if old_parent == parent {
            return self;
        }
        let ident = self.ident(db);
        PreciseTermSubentity::new(db, parent, ident)
    }
}
