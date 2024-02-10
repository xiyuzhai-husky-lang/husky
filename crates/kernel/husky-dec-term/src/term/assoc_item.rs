use super::*;
use husky_coword::Ident;

#[salsa::interned(db = DecTermDb, jar = DecTermJar)]
pub struct DecAssocItem {
    parent: DecTerm,
    ident: Ident,
}

impl DecAssocItem {
    #[inline(never)]
    pub(crate) fn display_fmt_with_db_and_ctx(
        self,
        _f: &mut std::fmt::Formatter<'_>,
        _db: &::salsa::Db,
        _ctx: &DecSvarNameMap,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl DecTermRewriteCopy for DecAssocItem {
    fn substitute_copy(self, db: &::salsa::Db, substitution: &DecTermSubstitution) -> Self {
        let old_parent = self.parent(db);
        let parent = old_parent.substitute_copy(db, substitution);
        if old_parent == parent {
            return self;
        }
        let ident = self.ident(db);
        DecAssocItem::new(db, parent, ident)
    }
}
