use crate::*;
use husky_word::Identifier;

#[salsa::interned(db = RawTermDb, jar = RawTermJar)]
pub struct RawTermAsTraitSubentity {
    parent: RawTerm,
    trai: RawTerm,
    ident: Identifier,
}

impl RawTermAsTraitSubentity {
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
    fn substitute_copy(self, db: &dyn RawTermDb, substituation: &RawTermSubstitution) -> Self
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
        RawTermAsTraitSubentity::new(db, parent, trai, ident)
    }
}
