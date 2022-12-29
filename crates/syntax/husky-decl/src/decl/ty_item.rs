mod assoc_ty;
mod assoc_val;
mod function;
mod memo;
mod method;

pub use assoc_ty::*;
pub use assoc_val::*;
pub use function::*;
pub use memo::*;
pub use method::*;

use crate::*;
use husky_ast::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TypeItemDecl {
    Function(TypeAssociatedFunctionDecl),
    Method(TypeMethodDecl),
    AlienType(TypeAssociatedTypeDecl),
    Value(TypeAssociatedValueDecl),
    Memo(TypeMemoDecl),
}

impl TypeItemDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            TypeItemDecl::Function(decl) => decl.ast_idx(db),
            TypeItemDecl::Method(decl) => decl.ast_idx(db),
            TypeItemDecl::AlienType(decl) => decl.ast_idx(db),
            TypeItemDecl::Value(decl) => decl.ast_idx(db),
            TypeItemDecl::Memo(decl) => decl.ast_idx(db),
        }
    }
}
