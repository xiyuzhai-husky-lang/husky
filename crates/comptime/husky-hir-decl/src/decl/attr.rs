mod derive;

pub use self::derive::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDeclDb)]
pub enum AttrHirDecl {
    Derive(DeriveAttrHirDecl),
}

impl AttrHirDecl {
    pub fn path(self, db: &dyn HirDeclDb) -> AttrPath {
        match self {
            AttrHirDecl::Derive(hir_decl) => hir_decl.path(db),
        }
    }
}
