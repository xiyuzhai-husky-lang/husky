use crate::*;
use husky_coword::Ident;

#[salsa::interned(db = DecTermDb, jar = DecTermJar)]
pub struct TypeAsTraitItemDecTerm {
    parent: DecTerm,
    trai: DecTerm,
    ident: Ident,
}

impl TypeAsTraitItemDecTerm {
    #[inline(never)]
    pub(crate) fn display_fmt_with_db_and_ctx(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &::salsa::Db,
        _ctx: &mut DecTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl DecTermRewriteCopy for TypeAsTraitItemDecTerm {
    fn substitute_copy(self, db: &::salsa::Db, substitution: &DecTermSubstitution) -> Self
    where
        Self: Copy,
    {
        let old_parent = self.parent(db);
        let parent = old_parent.substitute_copy(db, substitution);
        let old_trai = self.trai(db);
        let trai = old_trai.substitute_copy(db, substitution);
        if old_parent == parent && old_trai == trai {
            return self;
        }
        let ident = self.ident(db);
        Self::new(db, parent, trai, ident)
    }
}
