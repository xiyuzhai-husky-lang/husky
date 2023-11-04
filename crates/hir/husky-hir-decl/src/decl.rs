mod associated_item;
mod attr;
mod impl_block;
mod major_item;
mod submodule;
mod ty_variant;

pub use self::associated_item::*;
pub use self::attr::*;
pub use self::impl_block::*;
pub use self::major_item::*;
pub use self::submodule::*;
pub use self::ty_variant::*;

use crate::{db::*, parenate_parameter::*, *};
use husky_hir_eager_expr::helpers::hir_eager_expr_region;
use husky_hir_expr::source_map::HirExprSourceMap;
use husky_syn_decl::HasSynDecl;

pub trait HasHirDecl {
    type HirDecl;

    fn hir_decl(self, db: &dyn HirDeclDb) -> Option<Self::HirDecl>;
}

impl HasHirDecl for ItemPath {
    type HirDecl = HirDecl;

    fn hir_decl(self, db: &dyn HirDeclDb) -> Option<Self::HirDecl> {
        Some(match self {
            ItemPath::Submodule(path) => path.hir_decl(db)?.into(),
            ItemPath::MajorItem(path) => path.hir_decl(db)?.into(),
            ItemPath::AssociatedItem(path) => path.hir_decl(db)?.into(),
            ItemPath::TypeVariant(path) => path.hir_decl(db)?.into(),
            ItemPath::ImplBlock(path) => path.hir_decl(db)?.into(),
            ItemPath::Attr(_) => todo!(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = HirDeclDb)]
#[enum_class::from_variants]
pub enum HirDecl {
    Submodule(SubmoduleHirDecl),
    MajorItem(MajorItemHirDecl),
    ImplBlock(ImplBlockHirDecl),
    AssociatedItem(AssociatedItemHirDecl),
    TypeVariant(TypeVariantHirDecl),
    Attr(AttrHirDecl),
}

impl HirDecl {
    pub fn template_parameters<'a>(self, db: &'a dyn HirDeclDb) -> &'a [HirTemplateParameter] {
        match self {
            HirDecl::Submodule(_) => todo!(),
            HirDecl::MajorItem(decl) => decl.template_parameters(db),
            HirDecl::ImplBlock(decl) => decl.template_parameters(db),
            HirDecl::AssociatedItem(decl) => decl.template_parameters(db),
            HirDecl::TypeVariant(decl) => &[],
            HirDecl::Attr(_) => todo!(),
        }
    }

    // pub fn hir_expr_region(self, db: &dyn HirDeclDb) -> Option<HirExprRegion> {
    //     match self {
    //         HirDecl::Submodule(_) => None,
    //         HirDecl::MajorItem(decl) => decl.hir_expr_region(db).into(),
    //         HirDecl::ImplBlock(decl) => None,
    //         HirDecl::AssociatedItem(decl) => decl.hir_expr_region(db).into(),
    //         HirDecl::TypeVariant(_decl) => todo!(),
    //     }
    // }

    pub fn path(self, db: &dyn HirDeclDb) -> ItemPath {
        match self {
            HirDecl::Submodule(_) => todo!(),
            HirDecl::MajorItem(decl) => decl.path(db).into(),
            HirDecl::ImplBlock(decl) => decl.path(db).into(),
            HirDecl::AssociatedItem(decl) => decl.path(db).into(),
            HirDecl::TypeVariant(decl) => decl.path(db).into(),
            HirDecl::Attr(decl) => decl.path(db).into(),
        }
    }
}
