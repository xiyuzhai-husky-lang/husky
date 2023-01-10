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

    pub fn implicit_parameters(self, db: &dyn DeclDb) -> &[ImplicitParameterDecl] {
        match self {
            TraitItemDecl::Function(_) => todo!(),
            TraitItemDecl::Method(_) => todo!(),
            TraitItemDecl::AlienType(_) => todo!(),
            TraitItemDecl::Value(_) => todo!(),
        }
    }

    pub fn expr_sheet(self, db: &dyn DeclDb) -> ExprSheet {
        match self {
            TraitItemDecl::Function(_) => todo!(),
            TraitItemDecl::Method(_) => todo!(),
            TraitItemDecl::AlienType(_) => todo!(),
            TraitItemDecl::Value(_) => todo!(),
        }
    }

    pub fn path(self, db: &dyn DeclDb) -> TraitItemPath {
        match self {
            TraitItemDecl::Function(_) => todo!(),
            TraitItemDecl::Method(_) => todo!(),
            TraitItemDecl::AlienType(_) => todo!(),
            TraitItemDecl::Value(_) => todo!(),
        }
    }
}

impl<Db: DeclDb + ?Sized> salsa::DebugWithDb<Db> for TraitItemDecl {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}
