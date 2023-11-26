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
use ::version_stamp::HasVersionStamp;
use husky_hir_decl::parameter::parenate::eager::HirEagerParenateParameter;
use husky_hir_decl::parameter::template::HirTemplateParameter;
use husky_hir_eager_expr::helpers::hir_eager_body_with_expr_region;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDefnDb, jar = HirDefnJar)]
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
    pub fn hir_decl(self, db: &::salsa::Db,) -> HirDecl {
        match self {
            HirDefn::Submodule(hir_defn) => HirDecl::Submodule(hir_defn.hir_decl()),
            HirDefn::MajorItem(hir_defn) => hir_defn.hir_decl(db).into(),
            HirDefn::TypeVariant(hir_defn) => hir_defn.hir_decl(db).into(),
            HirDefn::ImplBlock(hir_decl) => hir_decl.hir_decl().into(),
            HirDefn::AssociatedItem(hir_defn) => hir_defn.hir_decl(db).into(),
            HirDefn::Attr(hir_defn) => hir_defn.hir_decl().into(),
        }
    }

    pub fn template_parameters<'a>(self, db: &'a ::salsa::Db) -> &'a [HirTemplateParameter] {
        self.hir_decl(db).template_parameters(db)
    }

    pub fn hir_expr_region(self, db: &::salsa::Db,) -> Option<HirExprRegion> {
        match self {
            HirDefn::Submodule(_) => None,
            HirDefn::MajorItem(hir_defn) => hir_defn.hir_expr_region(db),
            HirDefn::AssociatedItem(hir_defn) => hir_defn.hir_expr_region(db),
            HirDefn::TypeVariant(_defn) => None,
            HirDefn::ImplBlock(_) => None,
            HirDefn::Attr(_) => None,
        }
    }

    pub fn path(self, db: &::salsa::Db,) -> ItemPath {
        match self {
            HirDefn::MajorItem(hir_defn) => hir_defn.path(db).into(),
            HirDefn::AssociatedItem(hir_defn) => hir_defn.path(db).into(),
            HirDefn::TypeVariant(hir_defn) => hir_defn.path(db).into(),
            HirDefn::ImplBlock(hir_defn) => hir_defn.path(db).into(),
            HirDefn::Submodule(hir_defn) => hir_defn.path(db).into(),
            HirDefn::Attr(hir_defn) => hir_defn.path(db).into(),
        }
    }

    pub(crate) fn dependencies(self, db: &::salsa::Db,) -> Option<HirDefnDependencies> {
        match self {
            HirDefn::Submodule(_) => None,
            HirDefn::MajorItem(hir_defn) => Some(hir_defn.dependencies(db)),
            // ask its parent
            HirDefn::TypeVariant(hir_defn) => Some(hir_defn.dependencies(db)),
            HirDefn::ImplBlock(hir_defn) => Some(hir_defn.dependencies(db)),
            HirDefn::AssociatedItem(hir_defn) => Some(hir_defn.dependencies(db)),
            HirDefn::Attr(_) => None,
        }
    }

    pub fn opt_version_stamp(self, db: &::salsa::Db,) -> Option<HirDefnVersionStamp> {
        match self {
            HirDefn::Submodule(_) => None,
            HirDefn::MajorItem(hir_defn) => Some(hir_defn.version_stamp(db)),
            HirDefn::TypeVariant(hir_defn) => Some(hir_defn.version_stamp(db)),
            HirDefn::ImplBlock(hir_defn) => Some(hir_defn.version_stamp(db)),
            HirDefn::AssociatedItem(hir_defn) => Some(hir_defn.version_stamp(db)),
            HirDefn::Attr(_) => None,
        }
    }
}

impl  HasVersionStamp<Db> for HirDefn
where
     + HirDefnDb,
{
    type VersionStamp = HirDefnVersionStamp;

    fn version_stamp(self, db: &::salsa::Db,) -> Self::VersionStamp {
        let db = <Db as salsa::DbWithJar<HirDefnJar>>::as_jar_db(db);
        self.opt_version_stamp(db).unwrap()
    }
}

pub trait HasHirDefn: Copy {
    type HirDefn;

    fn hir_defn(self, db: &::salsa::Db,) -> Option<Self::HirDefn>;
}

impl HasHirDefn for ItemPath {
    type HirDefn = HirDefn;

    fn hir_defn(self, db: &::salsa::Db,) -> Option<Self::HirDefn> {
        Some(match self {
            ItemPath::Submodule(_, path) => path.hir_defn(db)?.into(),
            ItemPath::MajorItem(path) => path.hir_defn(db)?.into(),
            ItemPath::ImplBlock(path) => path.hir_defn(db)?.into(),
            ItemPath::AssociatedItem(path) => path.hir_defn(db)?.into(),
            ItemPath::TypeVariant(_, path) => path.hir_defn(db)?.into(),
            ItemPath::Attr(_, _) => todo!(),
        })
    }
}
