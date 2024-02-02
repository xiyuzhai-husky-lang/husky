use crate::*;
use husky_coword::Ident;

#[salsa::interned(db = DeclarativeTermDb, jar = DeclarativeTermJar)]
pub struct AssociatedItemDeclarativeTerm {
    parent: DeclarativeTerm,
    ident: Ident,
}

impl AssociatedItemDeclarativeTerm {
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

impl DeclarativeTermRewriteCopy for AssociatedItemDeclarativeTerm {
    fn substitute_copy(self, db: &::salsa::Db, substitution: &DeclarativeTermSubstitution) -> Self {
        let old_parent = self.parent(db);
        let parent = old_parent.substitute_copy(db, substitution);
        if old_parent == parent {
            return self;
        }
        let ident = self.ident(db);
        AssociatedItemDeclarativeTerm::new(db, parent, ident)
    }
}
