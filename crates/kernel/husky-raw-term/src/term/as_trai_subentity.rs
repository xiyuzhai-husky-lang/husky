use crate::*;
use husky_word::Ident;

#[salsa::interned(db = RawTermDb, jar = RawTermJar)]
pub struct RawTermAsTraitSubentity {
    parent: RawTerm,
    trai: RawTerm,
    ident: Ident,
}

impl RawTermAsTraitSubentity {
    pub(crate) fn show_with_db_fmt(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &dyn RawTermDb,
        _ctx: &mut RawTermShowContext,
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
