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

    pub fn implicit_parameters(self, db: &dyn DeclDb) -> &[ImplicitParameterDecl] {
        match self {
            TypeItemDecl::Function(_) => todo!(),
            TypeItemDecl::Method(_) => todo!(),
            TypeItemDecl::AlienType(_) => todo!(),
            TypeItemDecl::Value(_) => todo!(),
            TypeItemDecl::Memo(_) => todo!(),
        }
    }

    pub fn expr_sheet(self, db: &dyn DeclDb) -> ExprSheet {
        match self {
            TypeItemDecl::Function(defn) => defn.expr_sheet(db),
            TypeItemDecl::Method(defn) => defn.expr_sheet(db),
            TypeItemDecl::AlienType(defn) => defn.expr_sheet(db),
            TypeItemDecl::Value(defn) => defn.expr_sheet(db),
            TypeItemDecl::Memo(defn) => defn.expr_sheet(db),
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

impl<Db: DeclDb + ?Sized> salsa::DebugWithDb<Db> for TypeItemDecl {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<DeclJar>>::as_jar_db(db);
        match self {
            TypeItemDecl::Function(decl) => f
                .debug_tuple("Function")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeItemDecl::Method(decl) => f
                .debug_tuple("Method")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeItemDecl::AlienType(decl) => f
                .debug_tuple("AlienType")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeItemDecl::Value(decl) => f
                .debug_tuple("Value")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeItemDecl::Memo(decl) => f
                .debug_tuple("Memo")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
        }
    }
}
