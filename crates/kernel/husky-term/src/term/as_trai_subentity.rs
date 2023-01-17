use crate::*;
use husky_word::Identifier;

#[salsa::interned(jar = TermJar)]
pub struct TermAsTraitSubentity {
    parent: Term,
    trai: Term,
    ident: Identifier,
}

impl From<TermAsTraitSubentity> for Term {
    fn from(val: TermAsTraitSubentity) -> Self {
        Term::AsTraitSubentity(val)
    }
}

impl TermRewriteCopy for TermAsTraitSubentity {
    fn substitute_copy(self, db: &dyn TermDb, substituation: &TermSubstitution) -> Self
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
        TermAsTraitSubentity::new(db, parent, trai, ident)
    }
}
