mod assoc_fn;
mod assoc_ty;
mod assoc_val;
mod method_ritchie;

pub use self::assoc_fn::*;
pub use self::assoc_ty::*;
pub use self::assoc_val::*;
pub use self::method_ritchie::*;

use super::*;

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TraitItemHirDecl {
    AssocFn(TraitAssocFnHirDecl),
    MethodFn(TraitMethodFnHirDecl),
    AssocType(TraitAssocTypeHirDecl),
    AssocVal(TraitAssocValHirDecl),
}

impl HasHirDecl for TraitItemPath {
    type HirDecl = TraitItemHirDecl;

    fn hir_decl(self, _db: &::salsa::Db) -> Option<Self::HirDecl> {
        // ad hoc
        None
    }
}

impl TraitItemHirDecl {
    pub fn path(self, _db: &::salsa::Db) -> TraitItemPath {
        match self {
            TraitItemHirDecl::AssocFn(_) => todo!(),
            TraitItemHirDecl::MethodFn(_) => todo!(),
            TraitItemHirDecl::AssocType(_) => todo!(),
            TraitItemHirDecl::AssocVal(_) => todo!(),
        }
    }

    pub fn template_parameters<'a>(
        self,
        _db: &'a ::salsa::Db,
    ) -> Option<&'a HirTemplateParameters> {
        match self {
            TraitItemHirDecl::AssocFn(_) => todo!(),
            TraitItemHirDecl::MethodFn(_) => todo!(),
            TraitItemHirDecl::AssocType(_) => todo!(),
            TraitItemHirDecl::AssocVal(_) => todo!(),
        }
    }

    pub fn hir_expr_region(self, db: &::salsa::Db) -> HirExprRegion {
        match self {
            TraitItemHirDecl::AssocFn(hir_decl) => hir_decl.hir_eager_expr_region(db).into(),
            TraitItemHirDecl::MethodFn(hir_decl) => hir_decl.hir_eager_expr_region(db).into(),
            TraitItemHirDecl::AssocType(hir_decl) => hir_decl.hir_eager_expr_region(db).into(),
            TraitItemHirDecl::AssocVal(hir_decl) => hir_decl.hir_eager_expr_region(db).into(),
        }
    }
}
