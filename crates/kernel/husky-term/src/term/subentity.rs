use crate::*;
use husky_word::Identifier;

#[salsa::interned(db = TermDb, jar = TermJar)]
pub struct TermSubentity {
    parent: Term,
    ident: Identifier,
}

impl From<TermSubentity> for Term {
    fn from(val: TermSubentity) -> Self {
        Term::Subentity(val)
    }
}

impl TermRewriteCopy for TermSubentity {
    fn substitute_copy(self, db: &dyn TermDb, substituation: &TermSubstitution) -> Self {
        let old_parent = self.parent(db);
        let parent = old_parent.substitute_copy(db, substituation);
        if old_parent == parent {
            return self;
        }
        let ident = self.ident(db);
        TermSubentity::new(db, parent, ident)
    }
}
