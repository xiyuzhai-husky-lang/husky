use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar)]
pub struct TypeAliasHirDefn {
    pub path: FugitivePath,
    pub hir_expr_region: HirEagerExprRegion,
}

impl TypeAliasHirDefn {
    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        ty_alias_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        ty_alias_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn ty_alias_hir_defn_dependencies(
    db: &dyn HirDefnDb,
    hir_defn: TypeAliasHirDefn,
) -> HirDefnDependencies {
    todo!()
}

#[salsa::tracked(jar = HirDefnJar)]
fn ty_alias_hir_defn_version_stamp(
    db: &dyn HirDefnDb,
    hir_defn: TypeAliasHirDefn,
) -> HirDefnVersionStamp {
    todo!()
}
