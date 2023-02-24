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
#[salsa::derive_debug_with_db(db = DeclDb)]
pub enum TypeItemDecl {
    Function(TypeAssociatedFunctionDecl),
    Method(TypeMethodDecl),
    AlienType(TypeAssociatedTypeDecl),
    Value(TypeAssociatedValueDecl),
    Memo(TypeMemoDecl),
}

impl From<TypeMemoDecl> for TypeItemDecl {
    fn from(v: TypeMemoDecl) -> Self {
        Self::Memo(v)
    }
}

impl From<TypeAssociatedFunctionDecl> for TypeItemDecl {
    fn from(v: TypeAssociatedFunctionDecl) -> Self {
        Self::Function(v)
    }
}

impl From<TypeAssociatedValueDecl> for TypeItemDecl {
    fn from(v: TypeAssociatedValueDecl) -> Self {
        Self::Value(v)
    }
}

impl From<TypeAssociatedTypeDecl> for TypeItemDecl {
    fn from(v: TypeAssociatedTypeDecl) -> Self {
        Self::AlienType(v)
    }
}

impl From<TypeMethodDecl> for TypeItemDecl {
    fn from(v: TypeMethodDecl) -> Self {
        Self::Method(v)
    }
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

    pub fn implicit_parameters<'a>(
        self,
        db: &'a dyn DeclDb,
    ) -> DeclExprResultRef<'a, &'a [ImplicitParameterDecl]> {
        match self {
            TypeItemDecl::Function(_) => todo!(),
            TypeItemDecl::Method(_) => todo!(),
            TypeItemDecl::AlienType(_) => todo!(),
            TypeItemDecl::Value(_) => todo!(),
            TypeItemDecl::Memo(_) => todo!(),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> ExprRegion {
        match self {
            TypeItemDecl::Function(defn) => defn.expr_region(db),
            TypeItemDecl::Method(defn) => defn.expr_region(db),
            TypeItemDecl::AlienType(defn) => defn.expr_region(db),
            TypeItemDecl::Value(defn) => defn.expr_region(db),
            TypeItemDecl::Memo(defn) => defn.expr_region(db),
        }
    }

    pub fn path(self, db: &dyn DeclDb) -> Option<TypeItemPath> {
        match self {
            TypeItemDecl::Function(_) => todo!(),
            TypeItemDecl::Method(defn) => defn.path(db),
            TypeItemDecl::AlienType(_) => todo!(),
            TypeItemDecl::Value(_) => todo!(),
            TypeItemDecl::Memo(defn) => defn.path(db),
        }
    }
}
