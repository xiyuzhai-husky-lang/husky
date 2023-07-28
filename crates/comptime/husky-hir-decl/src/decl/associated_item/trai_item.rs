mod associated_fn;
mod associated_ty;
mod associated_val;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::method_fn::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[enum_class::from_variants]
pub enum TraitItemHirDecl {
    AssociatedFn(TraitAssociatedFnHirDecl),
    MethodFn(TraitMethodFnHirDecl),
    AssociatedType(TraitAssociatedTypeHirDecl),
    AssociatedVal(TraitAssociatedValHirDecl),
}

impl HasHirDecl for TraitItemPath {
    type HirDecl = TraitItemHirDecl;

    fn hir_decl(self, db: &dyn HirDeclDb) -> Self::HirDecl {
        todo!()
    }
}

impl TraitItemHirDecl {
    pub fn path(self, _db: &dyn HirDeclDb) -> TraitItemPath {
        match self {
            TraitItemHirDecl::AssociatedFn(_) => todo!(),
            TraitItemHirDecl::MethodFn(_) => todo!(),
            TraitItemHirDecl::AssociatedType(_) => todo!(),
            TraitItemHirDecl::AssociatedVal(_) => todo!(),
        }
    }

    pub fn template_parameters<'a>(
        self,
        _db: &'a dyn HirDeclDb,
    ) -> &'a [EtherealTemplateParameter] {
        match self {
            TraitItemHirDecl::AssociatedFn(_) => todo!(),
            TraitItemHirDecl::MethodFn(_) => todo!(),
            TraitItemHirDecl::AssociatedType(_) => todo!(),
            TraitItemHirDecl::AssociatedVal(_) => todo!(),
        }
    }

    pub fn hir_expr_region(self, db: &dyn HirDeclDb) -> HirExprRegion {
        match self {
            TraitItemHirDecl::AssociatedFn(hir_decl) => hir_decl.hir_expr_region(db).into(),
            TraitItemHirDecl::MethodFn(hir_decl) => hir_decl.hir_expr_region(db).into(),
            TraitItemHirDecl::AssociatedType(hir_decl) => hir_decl.hir_expr_region(db).into(),
            TraitItemHirDecl::AssociatedVal(hir_decl) => hir_decl.hir_expr_region(db).into(),
        }
    }
}
