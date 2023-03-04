use crate::*;
use husky_word::Identifier;

#[salsa::interned(db = PreciseTermDb, jar = PreciseTermJar)]
pub struct PreciseTermAsTraitSubentity {
    parent: PreciseTerm,
    trai: PreciseTerm,
    ident: Identifier,
}

impl PreciseTermAsTraitSubentity {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn PreciseTermDb,
        ctx: &mut PreciseTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl PreciseTermRewriteCopy for PreciseTermAsTraitSubentity {
    fn substitute_copy(
        self,
        db: &dyn PreciseTermDb,
        substituation: &PreciseTermSubstitution,
    ) -> Self
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
        PreciseTermAsTraitSubentity::new(db, parent, trai, ident)
    }
}
