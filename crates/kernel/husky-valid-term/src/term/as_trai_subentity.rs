use super::*;
use husky_word::Identifier;

#[salsa::interned(db = ValidTermDb, jar = ValidTermJar)]
pub struct ValidTermAsTraitSubentity {
    parent: ValidTerm,
    trai: ValidTerm,
    ident: Identifier,
}

impl ValidTermAsTraitSubentity {
    pub fn from_precise(db: &dyn ValidTermDb, precise_term: PreciseTermAsTraitSubentity) -> Self {
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

impl ValidTermRewriteCopy for ValidTermAsTraitSubentity {
    fn substitute_copy(self, db: &dyn ValidTermDb, substituation: &ValidTermSubstitution) -> Self
    where
        Self: Copy,
    {
        let old_parent = self.parent(db);
        let parent = old_parent.substitute_copy(db, substituation);
        let old_trai = self.trai(db);
        let trai = old_trai.substitute_copy(db, substituation);
        if old_parent == parent && old_trai == trai {
            return self;
        }
        let ident = self.ident(db);
        ValidTermAsTraitSubentity::new(db, parent, trai, ident)
    }
}
