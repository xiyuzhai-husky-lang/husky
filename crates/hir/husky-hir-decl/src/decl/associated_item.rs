mod trai_for_ty_item;
mod trai_item;
mod ty_item;

pub use self::trai_for_ty_item::*;
pub use self::trai_item::*;
pub use self::ty_item::*;

use super::*;

#[salsa::debug_with_db]
#[enum_class::from_variants]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AssociatedItemHirDecl {
    TypeItem(TypeItemHirDecl),
    TraitItem(TraitItemHirDecl),
    TraitForTypeItem(TraitForTypeItemHirDecl),
}

impl AssociatedItemHirDecl {
    pub fn path(self, db: &::salsa::Db) -> AssociatedItemPath {
        match self {
            AssociatedItemHirDecl::TypeItem(decl) => decl.path(db).into(),
            AssociatedItemHirDecl::TraitItem(decl) => decl.path(db).into(),
            AssociatedItemHirDecl::TraitForTypeItem(decl) => decl.path(db).into(),
        }
    }

    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> Option<&'a HirTemplateParameters> {
        match self {
            AssociatedItemHirDecl::TypeItem(decl) => decl.template_parameters(db),
            AssociatedItemHirDecl::TraitItem(decl) => decl.template_parameters(db),
            AssociatedItemHirDecl::TraitForTypeItem(decl) => decl.template_parameters(db),
        }
    }

    pub fn hir_expr_region(self, db: &::salsa::Db) -> HirExprRegion {
        match self {
            AssociatedItemHirDecl::TypeItem(decl) => decl.hir_expr_region(db),
            AssociatedItemHirDecl::TraitItem(decl) => decl.hir_expr_region(db),
            AssociatedItemHirDecl::TraitForTypeItem(decl) => decl.hir_expr_region(db),
        }
    }
}

impl HasHirDecl for AssociatedItemPath {
    type HirDecl = AssociatedItemHirDecl;

    fn hir_decl(self, db: &::salsa::Db) -> Option<Self::HirDecl> {
        match self {
            AssociatedItemPath::TypeItem(path) => path.hir_decl(db).map(Into::into),
            AssociatedItemPath::TraitItem(path) => path.hir_decl(db).map(Into::into),
            AssociatedItemPath::TraitForTypeItem(path) => path.hir_decl(db).map(Into::into),
        }
    }
}
