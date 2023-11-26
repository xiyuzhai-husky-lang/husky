use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar)]
pub struct TypeAliasHirDefn {
    pub path: FugitivePath,
    pub hir_decl: TypeAliasHirDecl,
    pub hir_eager_expr_region: Option<HirEagerExprRegion>,
}

impl From<TypeAliasHirDefn> for MajorItemHirDefn {
    fn from(hir_defn: TypeAliasHirDefn) -> Self {
        MajorItemHirDefn::Fugitive(hir_defn.into())
    }
}

impl From<TypeAliasHirDefn> for HirDefn {
    fn from(hir_defn: TypeAliasHirDefn) -> Self {
        HirDefn::MajorItem(hir_defn.into())
    }
}

impl TypeAliasHirDefn {
    pub(super) fn dependencies(self, db: &::salsa::Db) -> HirDefnDependencies {
        ty_alias_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &::salsa::Db) -> HirDefnVersionStamp {
        ty_alias_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn ty_alias_hir_defn_dependencies(
    db: &::salsa::Db,
    hir_defn: TypeAliasHirDefn,
) -> HirDefnDependencies {
    let mut builder = HirDefnDependenciesBuilder::new(hir_defn.path(db), db);
    let hir_decl = hir_defn.hir_decl(db);
    builder.add_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db));
    builder.add_hir_ty(hir_decl.ty(db));
    if let Some(hir_eager_expr_region) = hir_defn.hir_eager_expr_region(db) {
        builder.add_hir_eager_expr_region(hir_eager_expr_region);
    }
    builder.finish()
}

#[salsa::tracked(jar = HirDefnJar)]
fn ty_alias_hir_defn_version_stamp(
    db: &::salsa::Db,
    hir_defn: TypeAliasHirDefn,
) -> HirDefnVersionStamp {
    HirDefnVersionStamp::new(hir_defn, db)
}
