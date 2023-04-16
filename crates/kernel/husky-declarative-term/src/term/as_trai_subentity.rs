use crate::*;
use husky_word::Ident;

#[salsa::interned(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
pub struct DeclarativeTermAsTraitSubentity {
    parent: DeclarativeTerm,
    trai: DeclarativeTerm,
    ident: Ident,
}

impl DeclarativeTermAsTraitSubentity {
    pub(crate) fn show_with_db_fmt(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &dyn DeclarativeTermDb,
        _ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl DeclarativeTermRewriteCopy for DeclarativeTermAsTraitSubentity {
    fn substitute(
        self,
        db: &dyn DeclarativeTermDb,
        substituation: &DeclarativeTermSubstitution,
    ) -> Self
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
        DeclarativeTermAsTraitSubentity::new(db, parent, trai, ident)
    }
}
