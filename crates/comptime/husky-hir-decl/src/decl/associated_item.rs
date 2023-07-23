mod trai_for_ty_item;
mod trai_item;
mod ty_item;

pub use self::trai_for_ty_item::*;
pub use self::trai_item::*;
pub use self::ty_item::*;

use super::*;

type SmallVecImpl<T> = SmallVec<[T; 2]>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = HirDeclDb)]
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

    pub fn generic_parameters<'a>(self, db: &'a dyn HirDeclDb) -> &'a [EtherealGenericParameter] {
        match self {
            AssociatedItemHirDecl::TypeItem(decl) => decl.generic_parameters(db),
            AssociatedItemHirDecl::TraitItem(decl) => decl.generic_parameters(db),
            AssociatedItemHirDecl::TraitForTypeItem(_) => todo!(),
        }
    }

    pub fn syn_expr_region(self, db: &dyn HirDeclDb) -> HirExprRegion {
        match self {
            AssociatedItemHirDecl::TypeItem(decl) => decl.expr_region(db),
            AssociatedItemHirDecl::TraitItem(decl) => decl.expr_region(db),
            AssociatedItemHirDecl::TraitForTypeItem(decl) => decl.expr_region(db),
        }
    }
}
