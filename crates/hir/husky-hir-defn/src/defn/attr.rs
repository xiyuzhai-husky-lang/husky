use crate::*;
use husky_hir_decl::AttrHirDecl;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct AttrHirDefn {
    hir_decl: AttrHirDecl,
}

impl AttrHirDefn {
    pub fn path(self, db: &::salsa::Db) -> AttrItemPath {
        self.hir_decl.path(db)
    }

    pub fn hir_decl(&self) -> AttrHirDecl {
        self.hir_decl
    }
}
