use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar)]
pub struct UnitStructHirDefn {
    pub path: TypePath,
    pub hir_decl: UnitStructHirDecl,
}

impl From<UnitStructHirDefn> for MajorItemHirDefn {
    fn from(hir_defn: UnitStructHirDefn) -> Self {
        MajorItemHirDefn::Type(hir_defn.into())
    }
}

impl From<UnitStructHirDefn> for HirDefn {
    fn from(hir_defn: UnitStructHirDefn) -> Self {
        HirDefn::MajorItem(hir_defn.into())
    }
}

impl UnitStructHirDefn {
    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        unit_struct_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        unit_struct_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn unit_struct_hir_defn_dependencies(
    db: &::salsa::Db,
    hir_defn: UnitStructHirDefn,
) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    builder.finish()
}

#[salsa::tracked(jar = HirDefnJar)]
fn unit_struct_hir_defn_version_stamp(
    db: &::salsa::Db,
    hir_defn: UnitStructHirDefn,
) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
