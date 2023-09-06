use crate::*;
use husky_hir_decl::DecrHirDecl;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DecrHirDefn {
    hir_decl: DecrHirDecl,
}

impl DecrHirDefn {
    pub fn hir_decl(&self) -> DecrHirDecl {
        self.hir_decl
    }
}
