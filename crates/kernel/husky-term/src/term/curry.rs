pub use context::*;

use crate::*;

/// representing term `x -> y`
#[salsa::interned(jar = TermJar)]
pub struct TermCurry {
    pub x: Term,
    pub y: Term,
    // ty: Term,
}

impl TermRewriteCopy for TermCurry {
    fn substitute_copy(self, db: &dyn TermDb, substituation: &TermSubstitution) -> Self {
        todo!()
    }
}
