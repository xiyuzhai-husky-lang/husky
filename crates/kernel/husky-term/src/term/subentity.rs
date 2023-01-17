use crate::*;
use husky_word::Identifier;

#[salsa::interned(jar = TermJar)]
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
        todo!()
    }
}
