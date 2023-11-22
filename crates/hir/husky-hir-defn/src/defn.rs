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

use crate::*;
use husky_hir_decl::parameter::template::HirTemplateParameter;
use husky_hir_eager_expr::helpers::hir_eager_body_with_expr_region;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDefnDb)]
#[enum_class::from_variants]
pub enum ItemHirDefn {
    Submodule(SubmoduleHirDefn),
    MajorItem(MajorItemHirDefn),
    TypeVariant(TypeVariantHirDefn),
    ImplBlock(ImplBlockHirDefn),
    AssociatedItem(AssociatedItemHirDefn),
    Attr(AttrHirDefn),
}

impl ItemHirDefn {
    pub fn hir_decl(self, db: &dyn HirDefnDb) -> HirDecl {
        match self {
            ItemHirDefn::Submodule(hir_defn) => HirDecl::Submodule(hir_defn.hir_decl()),
            ItemHirDefn::MajorItem(hir_defn) => hir_defn.hir_decl(db).into(),
            ItemHirDefn::TypeVariant(hir_defn) => hir_defn.hir_decl(db).into(),
            ItemHirDefn::ImplBlock(hir_decl) => hir_decl.hir_decl().into(),
            ItemHirDefn::AssociatedItem(hir_defn) => hir_defn.hir_decl(db).into(),
            ItemHirDefn::Attr(hir_defn) => hir_defn.hir_decl().into(),
        }
    }

    pub fn template_parameters<'a>(self, db: &'a dyn HirDefnDb) -> &'a [HirTemplateParameter] {
        self.hir_decl(db).template_parameters(db)
    }

    pub fn hir_expr_region(self, db: &dyn HirDefnDb) -> Option<HirExprRegion> {
        match self {
            ItemHirDefn::Submodule(_) => None,
            ItemHirDefn::MajorItem(hir_defn) => hir_defn.hir_expr_region(db),
            ItemHirDefn::AssociatedItem(hir_defn) => hir_defn.hir_expr_region(db),
            ItemHirDefn::TypeVariant(_defn) => None,
            ItemHirDefn::ImplBlock(_) => None,
            ItemHirDefn::Attr(_) => None,
        }
    }

    pub fn path(self, db: &dyn HirDefnDb) -> ItemPath {
        match self {
            ItemHirDefn::MajorItem(hir_defn) => hir_defn.path(db).into(),
            ItemHirDefn::AssociatedItem(hir_defn) => hir_defn.path(db).into(),
            ItemHirDefn::TypeVariant(hir_defn) => hir_defn.path(db).into(),
            ItemHirDefn::ImplBlock(hir_defn) => hir_defn.path(db).into(),
            ItemHirDefn::Submodule(hir_defn) => hir_defn.path(db).into(),
            ItemHirDefn::Attr(hir_defn) => hir_defn.path(db).into(),
        }
    }

    pub(crate) fn dependencies(self, db: &dyn HirDefnDb) -> Option<ItemHirDefnDependencies> {
        match self {
            ItemHirDefn::Submodule(_) => None,
            ItemHirDefn::MajorItem(hir_defn) => match hir_defn {
                MajorItemHirDefn::Type(_) => todo!(),
                MajorItemHirDefn::Trait(_) => todo!(),
                MajorItemHirDefn::Fugitive(_) => todo!(),
            },
            // ask its parent
            ItemHirDefn::TypeVariant(_) => None,
            ItemHirDefn::ImplBlock(_) => None,
            ItemHirDefn::AssociatedItem(hir_defn) => Some(hir_defn.dependencies(db)),
            ItemHirDefn::Attr(_) => None,
        }
    }
}

pub trait HasHirDefn: Copy {
    type HirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Option<Self::HirDefn>;
}

impl HasHirDefn for ItemPath {
    type HirDefn = ItemHirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Option<Self::HirDefn> {
        Some(match self {
            ItemPath::Submodule(path) => path.hir_defn(db)?.into(),
            ItemPath::MajorItem(path) => path.hir_defn(db)?.into(),
            ItemPath::ImplBlock(path) => path.hir_defn(db)?.into(),
            ItemPath::AssociatedItem(path) => path.hir_defn(db)?.into(),
            ItemPath::TypeVariant(_) => todo!(),
            ItemPath::Attr(_) => todo!(),
        })
    }
}
