use crate::*;
use husky_coword::Ident;

#[salsa::interned(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
pub struct DeclarativeTermSubitem {
    parent: DeclarativeTerm,
    ident: Ident,
}

impl DeclarativeTermSubitem {
    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &dyn DeclarativeTermDb,
        _ctx: &mut DeclarativeTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl DeclarativeTermRewriteCopy for DeclarativeTermSubitem {
    fn substitute(
        self,
        db: &dyn DeclarativeTermDb,
        substituation: &DeclarativeTermSubstitution,
    ) -> Self {
        let old_parent = self.parent(db);
        let parent = old_parent.substitute(db, substituation);
        if old_parent == parent {
            return self;
        }
        let ident = self.ident(db);
        DeclarativeTermSubitem::new(db, parent, ident)
    }
}
