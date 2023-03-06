use super::*;
use husky_word::Identifier;

#[salsa::interned(db = RawTermDb, jar = RawTermJar)]
pub struct RawTermAsTraitSubentity {
    parent: RawTerm,
    trai: RawTerm,
    ident: Identifier,
}

impl RawTermAsTraitSubentity {
    pub fn from_raw_unchecked(
        db: &dyn RawTermDb,
        raw_term: RawTermAsTraitSubentity,
        raw_ty_expectation: TermTypeExpectation,
    ) -> RawTermResult<Self> {
        todo!()
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn RawTermDb,
        ctx: &mut RawTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl RawTermRewriteCopy for RawTermAsTraitSubentity {
    fn substitute(self, db: &dyn RawTermDb, substituation: &RawTermSubstitution) -> Self
    where
        Self: Copy,
    {
        let old_parent = self.parent(db);
        let parent = old_parent.substitute(db, substituation);
        let old_trai = self.trai(db);
        let trai = old_trai.substitute(db, substituation);
        if old_parent == parent && old_trai == trai {
            return self;
        }
        let ident = self.ident(db);
        RawTermAsTraitSubentity::new(db, parent, trai, ident)
    }
}
