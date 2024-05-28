pub mod trai_for_ty_item;
pub mod trai_item;
pub mod ty_item;

pub use self::trai_for_ty_item::*;
pub use self::trai_item::*;
pub use self::ty_item::*;

use super::*;
use husky_entity_path::path::assoc_item::AssocItemPath;

#[salsa::derive_debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AssocItemHirDecl {
    TypeItem(TypeItemHirDecl),
    TraitItem(TraitItemHirDecl),
    TraitForTypeItem(TraitForTypeItemHirDecl),
}

impl AssocItemHirDecl {
    pub fn path(self, db: &::salsa::Db) -> AssocItemPath {
        match self {
            AssocItemHirDecl::TypeItem(decl) => decl.path(db).into(),
            AssocItemHirDecl::TraitItem(decl) => decl.path(db).into(),
            AssocItemHirDecl::TraitForTypeItem(decl) => decl.path(db).into(),
        }
    }

    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> Option<&'a HirTemplateParameters> {
        match self {
            AssocItemHirDecl::TypeItem(decl) => decl.template_parameters(db),
            AssocItemHirDecl::TraitItem(decl) => decl.template_parameters(db),
            AssocItemHirDecl::TraitForTypeItem(decl) => decl.template_parameters(db),
        }
    }

    pub fn hir_expr_region(self, db: &::salsa::Db) -> HirExprRegion {
        match self {
            AssocItemHirDecl::TypeItem(decl) => decl.hir_expr_region(db),
            AssocItemHirDecl::TraitItem(decl) => decl.hir_expr_region(db),
            AssocItemHirDecl::TraitForTypeItem(decl) => decl.hir_expr_region(db),
        }
    }
}

impl HasHirDecl for AssocItemPath {
    type HirDecl = AssocItemHirDecl;

    fn hir_decl(self, db: &::salsa::Db) -> Option<Self::HirDecl> {
        match self {
            AssocItemPath::TypeItem(path) => path.hir_decl(db).map(Into::into),
            AssocItemPath::TraitItem(path) => path.hir_decl(db).map(Into::into),
            AssocItemPath::TraitForTypeItem(path) => path.hir_decl(db).map(Into::into),
        }
    }
}
