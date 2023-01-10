mod assoc_ty;
mod assoc_val;
mod function;
mod method;

pub use assoc_ty::*;
pub use assoc_val::*;
pub use function::*;
pub use method::*;

use super::*;
use husky_ast::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TypeAsTraitItemDecl {
    Function(TypeAsTraitAssociatedFunctionDecl),
    Method(TypeAsTraitMethodDecl),
    AlienType(TypeAsTraitAssociatedTypeDecl),
    Value(TypeAsTraitAssociatedValueDecl),
}

impl From<TypeAsTraitAssociatedValueDecl> for TypeAsTraitItemDecl {
    fn from(v: TypeAsTraitAssociatedValueDecl) -> Self {
        Self::Value(v)
    }
}

impl From<TypeAsTraitAssociatedTypeDecl> for TypeAsTraitItemDecl {
    fn from(v: TypeAsTraitAssociatedTypeDecl) -> Self {
        Self::AlienType(v)
    }
}

impl From<TypeAsTraitMethodDecl> for TypeAsTraitItemDecl {
    fn from(v: TypeAsTraitMethodDecl) -> Self {
        Self::Method(v)
    }
}

impl From<TypeAsTraitAssociatedFunctionDecl> for TypeAsTraitItemDecl {
    fn from(v: TypeAsTraitAssociatedFunctionDecl) -> Self {
        Self::Function(v)
    }
}

impl TypeAsTraitItemDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            TypeAsTraitItemDecl::Function(decl) => decl.ast_idx(db),
            TypeAsTraitItemDecl::Method(decl) => decl.ast_idx(db),
            TypeAsTraitItemDecl::AlienType(decl) => decl.ast_idx(db),
            TypeAsTraitItemDecl::Value(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters(self, db: &dyn DeclDb) -> &[ImplicitParameterDecl] {
        match self {
            TypeAsTraitItemDecl::Function(_) => todo!(),
            TypeAsTraitItemDecl::Method(_) => todo!(),
            TypeAsTraitItemDecl::AlienType(_) => todo!(),
            TypeAsTraitItemDecl::Value(_) => todo!(),
        }
    }

    pub fn expr_sheet(self, db: &dyn DeclDb) -> ExprSheet {
        match self {
            TypeAsTraitItemDecl::Function(decl) => decl.expr_sheet(db),
            TypeAsTraitItemDecl::Method(decl) => decl.expr_sheet(db),
            TypeAsTraitItemDecl::AlienType(decl) => decl.expr_sheet(db),
            TypeAsTraitItemDecl::Value(decl) => decl.expr_sheet(db),
        }
    }

    pub fn path(self, db: &dyn DeclDb) -> Option<TypeAsTraitItemPath> {
        match self {
            TypeAsTraitItemDecl::Function(_) => todo!(),
            TypeAsTraitItemDecl::Method(decl) => decl.path(db),
            TypeAsTraitItemDecl::AlienType(_) => todo!(),
            TypeAsTraitItemDecl::Value(_) => todo!(),
        }
    }
}

impl<Db: DeclDb + ?Sized> salsa::DebugWithDb<Db> for TypeAsTraitItemDecl {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<DeclJar>>::as_jar_db(db);
        match self {
            TypeAsTraitItemDecl::Function(decl) => f
                .debug_tuple("Function")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeAsTraitItemDecl::Method(decl) => f
                .debug_tuple("Method")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeAsTraitItemDecl::AlienType(decl) => f
                .debug_tuple("AlienType")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
            TypeAsTraitItemDecl::Value(decl) => f
                .debug_tuple("Value")
                .field(&decl.debug_with(db, include_all_fields))
                .finish(),
        }
    }
}
