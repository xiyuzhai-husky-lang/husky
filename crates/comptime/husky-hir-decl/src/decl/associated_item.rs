mod trai_for_ty_item;
mod trai_item;
mod ty_item;

pub use self::trai_for_ty_item::*;
pub use self::trai_item::*;
pub use self::ty_item::*;

use super::*;

type SmallVecImpl<T> = SmallVec<[T; 2]>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDeclDb)]
#[enum_class::from_variants]
pub enum AssociatedItemHirDecl {
    TypeItem(TypeItemHirDecl),
    TraitItem(TraitItemHirDecl),
    TraitForTypeItem(TraitForTypeItemHirDecl),
}

impl AssociatedItemHirDecl {
    pub fn path(self, db: &dyn HirDeclDb) -> AssociatedItemPath {
        match self {
            AssociatedItemHirDecl::TypeItem(decl) => decl.path(db).into(),
            AssociatedItemHirDecl::TraitItem(decl) => decl.path(db).into(),
            AssociatedItemHirDecl::TraitForTypeItem(decl) => decl.path(db).into(),
        }
    }

    pub fn template_parameters<'a>(self, db: &'a dyn HirDeclDb) -> &'a [HirTemplateParameter] {
        match self {
            AssociatedItemHirDecl::TypeItem(decl) => decl.template_parameters(db),
            AssociatedItemHirDecl::TraitItem(decl) => decl.template_parameters(db),
            AssociatedItemHirDecl::TraitForTypeItem(_) => todo!(),
        }
    }

    pub fn hir_expr_region(self, db: &dyn HirDeclDb) -> HirExprRegion {
        match self {
            AssociatedItemHirDecl::TypeItem(decl) => decl.hir_expr_region(db),
            AssociatedItemHirDecl::TraitItem(decl) => decl.hir_expr_region(db),
            AssociatedItemHirDecl::TraitForTypeItem(decl) => decl.hir_expr_region(db),
        }
    }
}

impl HasHirDecl for AssociatedItemPath {
    type HirDecl = AssociatedItemHirDecl;

    fn hir_decl(self, db: &dyn HirDeclDb) -> Option<Self::HirDecl> {
        todo!()
    }
}
