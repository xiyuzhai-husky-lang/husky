mod associated_fn;
mod associated_ty;
mod associated_val;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::method_fn::*;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[enum_class::from_variants]
pub enum TraitForTypeItemHirDecl {
    AssociatedFn(TraitForTypeAssociatedFnHirDecl),
    AssociatedType(TraitForTypeAssociatedTypeHirDecl),
    AssociatedVal(TraitForTypeAssociatedValHirDecl),
    MethodFn(TraitForTypeMethodFnHirDecl),
}

impl TraitForTypeItemHirDecl {
    pub fn path(self, db: &dyn HirDeclDb) -> TraitForTypeItemPath {
        match self {
            TraitForTypeItemHirDecl::AssociatedFn(decl) => decl.path(db),
            TraitForTypeItemHirDecl::MethodFn(decl) => decl.path(db),
            TraitForTypeItemHirDecl::AssociatedType(decl) => decl.path(db),
            TraitForTypeItemHirDecl::AssociatedVal(decl) => decl.path(db),
        }
    }

    pub fn generic_parameters<'a>(self, db: &'a dyn HirDeclDb) -> &'a [EtherealGenericParameter] {
        match self {
            TraitForTypeItemHirDecl::AssociatedFn(decl) => decl.generic_parameters(db),
            TraitForTypeItemHirDecl::MethodFn(decl) => decl.generic_parameters(db),
            TraitForTypeItemHirDecl::AssociatedType(decl) => decl.generic_parameters(db),
            TraitForTypeItemHirDecl::AssociatedVal(_) => &[],
        }
    }

    pub fn hir_expr_region(self, db: &dyn HirDeclDb) -> HirExprRegion {
        match self {
            TraitForTypeItemHirDecl::AssociatedFn(decl) => decl.hir_expr_region(db).into(),
            TraitForTypeItemHirDecl::MethodFn(decl) => decl.hir_expr_region(db).into(),
            TraitForTypeItemHirDecl::AssociatedType(decl) => decl.hir_expr_region(db).into(),
            TraitForTypeItemHirDecl::AssociatedVal(decl) => decl.hir_expr_region(db).into(),
        }
    }
}

impl HasHirDecl for TraitForTypeItemPath {
    type HirDecl = TraitForTypeItemHirDecl;

    fn hir_decl(self, db: &dyn HirDeclDb) -> Self::HirDecl {
        trai_for_ty_item_hir_decl(db, self)
    }
}

#[salsa::tracked(jar = HirDeclJar)]
fn trai_for_ty_item_hir_decl(
    db: &dyn HirDeclDb,
    path: TraitForTypeItemPath,
) -> TraitForTypeItemHirDecl {
    todo!()
}
