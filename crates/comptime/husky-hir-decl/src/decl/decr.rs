mod derive;

pub use self::derive::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDeclDb)]
pub enum DecrHirDecl {
    Derive(DeriveDecrHirDecl),
}

impl DecrHirDecl {
    pub fn path(self, db: &dyn HirDeclDb) -> DecrPath {
        match self {
            DecrHirDecl::Derive(hir_decl) => hir_decl.path(db),
        }
    }
}
