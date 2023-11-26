mod derive;

pub use self::derive::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDeclDb, jar = HirDeclJar)]
pub enum AttrHirDecl {
    Derive(DeriveAttrHirDecl),
}

impl AttrHirDecl {
    pub fn path(self, db: &dyn HirDeclDb) -> AttrItemPath {
        match self {
            AttrHirDecl::Derive(hir_decl) => hir_decl.path(db),
        }
    }
}
