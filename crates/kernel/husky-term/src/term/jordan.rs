pub use context::*;

use crate::*;

/// representing term `x -> y`
#[salsa::interned(jar = TermJar)]
pub struct TermJordan {
    pub kind: TermJordanKind,
    #[return_ref]
    pub params: Vec<TermJordanParameter>,
    pub y: Term,
    // ty: Term,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct TermJordanParameter {}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TermJordanKind {
    Fn,
    FnMut,
}

impl Into<Term> for TermJordan {
    fn into(self) -> Term {
        Term::Jordan(self)
    }
}

impl TermRewriteCopy for TermJordan {
    fn substitute_copy(self, db: &dyn TermDb, substituation: &TermSubstitution) -> Self {
        todo!()
    }
}
