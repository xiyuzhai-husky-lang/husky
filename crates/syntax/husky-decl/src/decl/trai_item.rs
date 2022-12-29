mod assoc_ty;
mod assoc_val;
mod function;
mod method;

pub use assoc_ty::*;
pub use assoc_val::*;
pub use function::*;
pub use method::*;

use crate::*;
use husky_ast::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TraitItemDecl {
    Function(TraitAssociatedFunctionDecl),
    Method(TraitMethodDecl),
    AlienType(TraitAssociatedTypeDecl),
    Value(TraitAssociatedValueDecl),
}

impl TraitItemDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            TraitItemDecl::Function(decl) => decl.ast_idx(db),
            TraitItemDecl::Method(decl) => decl.ast_idx(db),
            TraitItemDecl::AlienType(decl) => decl.ast_idx(db),
            TraitItemDecl::Value(decl) => decl.ast_idx(db),
        }
    }
}
