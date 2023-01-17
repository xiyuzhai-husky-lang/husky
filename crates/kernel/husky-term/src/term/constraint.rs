use crate::*;

#[salsa::interned(jar = TermJar)]
pub struct TermTraitConstraint {
    ty: Term,
    trai: Term,
}

impl TermRewriteCopy for TermTraitConstraint {
    fn substitute_copy(self, db: &dyn TermDb, substituation: &TermSubstitution) -> Self {
        todo!()
    }
}

impl From<TermTraitConstraint> for Term {
    fn from(v: TermTraitConstraint) -> Self {
        Self::TraitConstraint(v)
    }
}
