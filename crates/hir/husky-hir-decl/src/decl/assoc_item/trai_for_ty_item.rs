mod assoc_fn;
mod assoc_ty;
mod assoc_val;
mod method_fn;

use husky_syn_decl::TraitForTypeItemSynDecl;

pub use self::assoc_fn::*;
pub use self::assoc_ty::*;
pub use self::assoc_val::*;
pub use self::method_fn::*;

use super::*;

#[salsa::debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TraitForTypeItemHirDecl {
    AssocFn(TraitForTypeAssocFnHirDecl),
    AssocType(TraitForTypeAssocTypeHirDecl),
    AssocVal(TraitForTypeAssocValHirDecl),
    MethodFn(TraitForTypeMethodFnHirDecl),
}

impl TraitForTypeItemHirDecl {
    pub fn path(self, db: &::salsa::Db) -> TraitForTypeItemPath {
        match self {
            TraitForTypeItemHirDecl::AssocFn(decl) => decl.path(db),
            TraitForTypeItemHirDecl::MethodFn(decl) => decl.path(db),
            TraitForTypeItemHirDecl::AssocType(decl) => decl.path(db),
            TraitForTypeItemHirDecl::AssocVal(decl) => decl.path(db),
        }
    }

    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> Option<&'a HirTemplateParameters> {
        match self {
            TraitForTypeItemHirDecl::AssocFn(decl) => Some(decl.template_parameters(db)),
            TraitForTypeItemHirDecl::MethodFn(decl) => Some(decl.template_parameters(db)),
            TraitForTypeItemHirDecl::AssocType(decl) => Some(decl.template_parameters(db)),
            TraitForTypeItemHirDecl::AssocVal(_) => None,
        }
    }

    pub fn hir_expr_region(self, db: &::salsa::Db) -> HirExprRegion {
        match self {
            TraitForTypeItemHirDecl::AssocFn(decl) => decl.hir_eager_expr_region(db).into(),
            TraitForTypeItemHirDecl::MethodFn(decl) => decl.hir_eager_expr_region(db).into(),
            TraitForTypeItemHirDecl::AssocType(decl) => decl.hir_eager_expr_region(db).into(),
            TraitForTypeItemHirDecl::AssocVal(decl) => decl.hir_expr_region(db),
        }
    }
}

impl HasHirDecl for TraitForTypeItemPath {
    type HirDecl = TraitForTypeItemHirDecl;

    fn hir_decl(self, db: &::salsa::Db) -> Option<Self::HirDecl> {
        trai_for_ty_item_hir_decl(db, self)
    }
}

#[salsa::tracked(jar = HirDeclJar)]
fn trai_for_ty_item_hir_decl(
    db: &::salsa::Db,
    path: TraitForTypeItemPath,
) -> Option<TraitForTypeItemHirDecl> {
    match path.syn_decl(db).expect("ok") {
        TraitForTypeItemSynDecl::AssocFn(syn_decl) => {
            Some(TraitForTypeAssocFnHirDecl::from_syn(path, syn_decl, db).into())
        }
        TraitForTypeItemSynDecl::MethodFn(method_decl) => {
            Some(TraitForTypeMethodFnHirDecl::from_syn(path, method_decl, db).into())
        }
        TraitForTypeItemSynDecl::AssocType(assoc_type_decl) => {
            Some(TraitForTypeAssocTypeHirDecl::from_syn(path, assoc_type_decl, db).into())
        }
        TraitForTypeItemSynDecl::AssocVal(assoc_val_decl) => {
            Some(TraitForTypeAssocValHirDecl::from_syn(path, assoc_val_decl, db).into())
        }
    }
}
