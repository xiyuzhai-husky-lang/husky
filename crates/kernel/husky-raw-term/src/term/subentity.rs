use crate::*;
use husky_word::Identifier;

#[salsa::interned(db = RawTermDb, jar = RawTermJar)]
pub struct RawTermSubentity {
    parent: RawTerm,
    ident: Identifier,
}

impl RawTermSubentity {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn RawTermDb,
        ctx: &mut RawTermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl RawTermRewriteCopy for RawTermSubentity {
    fn substitute(self, db: &dyn RawTermDb, substituation: &RawTermSubstitution) -> Self {
        let old_parent = self.parent(db);
        let parent = old_parent.substitute(db, substituation);
        if old_parent == parent {
            return self;
        }
        let ident = self.ident(db);
        RawTermSubentity::new(db, parent, ident)
    }
}
