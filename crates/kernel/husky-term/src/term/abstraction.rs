use crate::*;

#[salsa::interned(jar = TermJar)]
pub struct TermAbstraction {
    x: TermSymbol,
    m: Term,
}

impl TermAbstraction {
    pub fn ty(&self) -> Term {
        todo!()
    }

    // pub fn universe(&self) -> TermUniverseLevel {
    //     todo!()
    // }
}

impl TermRewriteCopy for TermAbstraction {
    fn substitute_copy(self, db: &dyn TermDb, substituation: &TermSubstitution) -> Self {
        todo!()
    }
}

impl std::fmt::Display for TermAbstraction {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<TermAbstraction> for Term {
    fn from(val: TermAbstraction) -> Self {
        Term::Abstraction(val)
    }
}
