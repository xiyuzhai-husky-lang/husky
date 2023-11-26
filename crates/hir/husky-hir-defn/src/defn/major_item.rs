mod fugitive;
mod trai;
mod ty;

pub use self::fugitive::*;
pub use self::trai::*;
pub use self::ty::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDefnDb, jar = HirDefnJar)]
#[enum_class::from_variants]
pub enum MajorItemHirDefn {
    Type(TypeHirDefn),
    Trait(TraitHirDefn),
    Fugitive(FugitiveHirDefn),
}

impl MajorItemHirDefn {
    pub fn path(self, db: &::salsa::Db) -> MajorItemPath {
        match self {
            MajorItemHirDefn::Type(hir_defn) => hir_defn.path(db).into(),
            MajorItemHirDefn::Trait(hir_defn) => hir_defn.path(db).into(),
            MajorItemHirDefn::Fugitive(hir_defn) => hir_defn.path(db).into(),
        }
    }

    pub fn hir_decl(self, db: &::salsa::Db) -> MajorItemHirDecl {
        match self {
            MajorItemHirDefn::Type(hir_defn) => hir_defn.hir_decl(db).into(),
            MajorItemHirDefn::Trait(hir_defn) => hir_defn.hir_decl(db).into(),
            MajorItemHirDefn::Fugitive(hir_defn) => hir_defn.hir_decl(db).into(),
        }
    }

    pub fn hir_expr_region(self, db: &::salsa::Db) -> Option<HirExprRegion> {
        match self {
            MajorItemHirDefn::Type(_) | MajorItemHirDefn::Trait(_) => None,
            MajorItemHirDefn::Fugitive(hir_defn) => hir_defn.hir_expr_region(db),
        }
    }

    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        match self {
            MajorItemHirDefn::Type(hir_defn) => hir_defn.dependencies(db),
            MajorItemHirDefn::Trait(hir_defn) => hir_defn.dependencies(db),
            MajorItemHirDefn::Fugitive(hir_defn) => hir_defn.dependencies(db),
        }
    }

    pub fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        match self {
            MajorItemHirDefn::Type(hir_defn) => hir_defn.version_stamp(db),
            MajorItemHirDefn::Trait(hir_defn) => hir_defn.version_stamp(db),
            MajorItemHirDefn::Fugitive(hir_defn) => hir_defn.version_stamp(db),
        }
    }
}

impl HasHirDefn for MajorItemPath {
    type HirDefn = MajorItemHirDefn;

    fn hir_defn(self, db: &::salsa::Db) -> Option<Self::HirDefn> {
        Some(match self {
            MajorItemPath::Type(path) => path.hir_defn(db)?.into(),
            MajorItemPath::Fugitive(path) => path.hir_defn(db)?.into(),
            MajorItemPath::Trait(path) => path.hir_defn(db)?.into(),
        })
    }
}
