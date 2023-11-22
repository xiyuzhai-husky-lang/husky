use super::*;
use husky_entity_syn_tree::HasTypeVariantPaths;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = pub(super) new)]
pub struct EnumHirDefn {
    pub path: TypePath,
    pub hir_decl: EnumTypeHirDecl,
}

impl From<EnumHirDefn> for MajorItemHirDefn {
    fn from(hir_defn: EnumHirDefn) -> Self {
        MajorItemHirDefn::Type(hir_defn.into())
    }
}

impl From<EnumHirDefn> for HirDefn {
    fn from(hir_defn: EnumHirDefn) -> Self {
        HirDefn::MajorItem(hir_defn.into())
    }
}

impl EnumHirDefn {
    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        enum_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        enum_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn enum_hir_defn_dependencies(db: &dyn HirDefnDb, hir_defn: EnumHirDefn) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    for &(_, ty_variant_path) in hir_decl.path(db).ty_variant_paths(db) {
        builder.add_item_path(ty_variant_path)
    }
    builder.finish()
}

#[salsa::tracked(jar = HirDefnJar)]
fn enum_hir_defn_version_stamp(db: &dyn HirDefnDb, hir_defn: EnumHirDefn) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
