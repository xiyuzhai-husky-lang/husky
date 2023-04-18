mod associated_fn;
mod associated_ty;
mod associated_val;
mod method_fn;

pub use associated_fn::*;
pub use associated_ty::*;
pub use associated_val::*;
pub use method_fn::*;

use super::*;
use husky_ast::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum TraitItemDecl {
    AssociatedFunction(TraitAssociatedFunctionDecl),
    Method(TraitMethodFnDecl),
    AssociatedType(TraitAssociatedTypeDecl),
    Value(TraitAssociatedValueDecl),
}

impl TraitItemDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            TraitItemDecl::AssociatedFunction(decl) => decl.ast_idx(db),
            TraitItemDecl::Method(decl) => decl.ast_idx(db),
            TraitItemDecl::AssociatedType(decl) => decl.ast_idx(db),
            TraitItemDecl::Value(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(self, _db: &'a dyn DeclDb) -> &'a [ImplicitParameterDecl] {
        match self {
            TraitItemDecl::AssociatedFunction(_) => todo!(),
            TraitItemDecl::Method(_) => todo!(),
            TraitItemDecl::AssociatedType(_) => todo!(),
            TraitItemDecl::Value(_) => todo!(),
        }
    }

    pub fn expr_region(self, _db: &dyn DeclDb) -> ExprRegion {
        match self {
            TraitItemDecl::AssociatedFunction(_) => todo!(),
            TraitItemDecl::Method(_) => todo!(),
            TraitItemDecl::AssociatedType(_) => todo!(),
            TraitItemDecl::Value(_) => todo!(),
        }
    }

    pub fn path(self, _db: &dyn DeclDb) -> TraitItemPath {
        match self {
            TraitItemDecl::AssociatedFunction(_) => todo!(),
            TraitItemDecl::Method(_) => todo!(),
            TraitItemDecl::AssociatedType(_) => todo!(),
            TraitItemDecl::Value(_) => todo!(),
        }
    }
}

impl<'a> DeclParseContext<'a> {}
