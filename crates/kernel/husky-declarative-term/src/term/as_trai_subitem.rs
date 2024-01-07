use crate::*;
use husky_coword::Ident;

#[salsa::interned(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
pub struct DeclarativeTermAsTraitSubitem {
    parent: DeclarativeTerm,
    trai: DeclarativeTerm,
    ident: Ident,
}

impl DeclarativeTermAsTraitSubitem {
    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &::salsa::Db,
        _ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl DeclarativeTermRewriteCopy for DeclarativeTermAsTraitSubitem {
    fn substitute_copy(self, db: &::salsa::Db, substituation: &DeclarativeTermSubstitution) -> Self
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
        Self::new(db, parent, trai, ident)
    }
}
