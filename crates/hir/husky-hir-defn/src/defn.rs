mod associated_item;
mod attr;
mod impl_block;
mod module_item;
mod submodule;
mod ty_variant;

pub use self::associated_item::*;
pub use self::attr::*;
pub use self::impl_block::*;
pub use self::module_item::*;
pub use self::submodule::*;
pub use self::ty_variant::*;

use crate::*;
use husky_hir_eager_expr::helpers::hir_eager_body_with_expr_region;
use husky_hir_ty::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDefnDb)]
#[enum_class::from_variants]
pub enum HirDefn {
    Submodule(SubmoduleHirDefn),
    MajorItem(MajorItemHirDefn),
    TypeVariant(TypeVariantHirDefn),
    ImplBlock(ImplBlockHirDefn),
    AssociatedItem(AssociatedItemHirDefn),
    Attr(AttrHirDefn),
}

impl HirDefn {
    pub fn hir_decl(self, db: &dyn HirDefnDb) -> HirDecl {
        match self {
            HirDefn::Submodule(hir_defn) => HirDecl::Submodule(hir_defn.hir_decl()),
            HirDefn::MajorItem(hir_defn) => hir_defn.hir_decl(db).into(),
            HirDefn::TypeVariant(hir_defn) => hir_defn.hir_decl(db).into(),
            HirDefn::ImplBlock(hir_decl) => hir_decl.hir_decl().into(),
            HirDefn::AssociatedItem(hir_defn) => hir_defn.hir_decl(db).into(),
            HirDefn::Attr(hir_defn) => hir_defn.hir_decl().into(),
        }
    }

    pub fn template_parameters<'a>(self, db: &'a dyn HirDefnDb) -> &'a [HirTemplateParameter] {
        self.hir_decl(db).template_parameters(db)
    }

    pub fn hir_expr_region(self, db: &dyn HirDefnDb) -> Option<HirExprRegion> {
        match self {
            HirDefn::Submodule(_) => None,
            HirDefn::MajorItem(hir_defn) => hir_defn.hir_expr_region(db),
            HirDefn::AssociatedItem(hir_defn) => hir_defn.hir_expr_region(db),
            HirDefn::TypeVariant(_defn) => None,
            HirDefn::ImplBlock(_) => None,
            HirDefn::Attr(_) => None,
        }
    }
}

impl HirDefn {
    pub fn path(self, db: &dyn HirDefnDb) -> ItemPath {
        match self {
            HirDefn::MajorItem(hir_defn) => hir_defn.path(db).into(),
            HirDefn::AssociatedItem(hir_defn) => hir_defn.path(db).into(),
            HirDefn::TypeVariant(hir_defn) => hir_defn.path(db).into(),
            HirDefn::ImplBlock(hir_defn) => hir_defn.path(db).into(),
            HirDefn::Submodule(hir_defn) => hir_defn.path(db).into(),
            HirDefn::Attr(hir_defn) => hir_defn.path(db).into(),
        }
    }
}

pub trait HasHirDefn: Copy {
    type HirDefn;

    fn hir_defn(self, db: &dyn HirDefnDb) -> Option<Self::HirDefn>;
}

impl HasHirDefn for ItemPath {
    type HirDefn = HirDefn;

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
