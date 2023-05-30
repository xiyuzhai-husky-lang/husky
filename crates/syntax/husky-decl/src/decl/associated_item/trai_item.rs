mod associated_fn;
mod associated_ty;
mod associated_val;
mod memoized_field;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::method_fn::*;

use super::*;
use husky_ast::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum TraitItemDecl {
    AssociatedFn(TraitAssociatedFnDecl),
    MethodFn(TraitMethodFnDecl),
    AssociatedType(TraitAssociatedTypeDecl),
    AssociatedVal(TraitAssociatedValDecl),
}

impl TraitItemDecl {
    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            TraitItemDecl::AssociatedFn(decl) => decl.ast_idx(db),
            TraitItemDecl::MethodFn(decl) => decl.ast_idx(db),
            TraitItemDecl::AssociatedType(decl) => decl.ast_idx(db),
            TraitItemDecl::AssociatedVal(decl) => decl.ast_idx(db),
        }
    }

    pub fn implicit_parameters<'a>(
        self,
        _db: &'a dyn DeclDb,
    ) -> &'a [ImplicitParameterDeclPattern] {
        match self {
            TraitItemDecl::AssociatedFn(_) => todo!(),
            TraitItemDecl::MethodFn(_) => todo!(),
            TraitItemDecl::AssociatedType(_) => todo!(),
            TraitItemDecl::AssociatedVal(_) => todo!(),
        }
    }

    pub fn expr_region(self, _db: &dyn DeclDb) -> ExprRegion {
        match self {
            TraitItemDecl::AssociatedFn(_) => todo!(),
            TraitItemDecl::MethodFn(_) => todo!(),
            TraitItemDecl::AssociatedType(_) => todo!(),
            TraitItemDecl::AssociatedVal(_) => todo!(),
        }
    }

    pub fn path(self, _db: &dyn DeclDb) -> TraitItemPath {
        match self {
            TraitItemDecl::AssociatedFn(_) => todo!(),
            TraitItemDecl::MethodFn(_) => todo!(),
            TraitItemDecl::AssociatedType(_) => todo!(),
            TraitItemDecl::AssociatedVal(_) => todo!(),
        }
    }
}

impl<'a> DeclParseContext<'a> {}
