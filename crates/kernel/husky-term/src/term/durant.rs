pub use context::*;

use crate::*;

/// representing term `x -> y`
#[salsa::interned(jar = TermJar)]
pub struct TermDurant {
    pub kind: TermDurantKind,
    #[return_ref]
    pub params: Vec<TermDurantParameter>,
    pub y: Term,
    // ty: Term,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct TermDurantParameter {}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TermDurantKind {
    Fn,
    FnMut,
}

impl TermRewriteCopy for TermDurant {
    fn substitute_copy(self, db: &dyn TermDb, substituation: &TermSubstitution) -> Self {
        todo!()
    }
}
