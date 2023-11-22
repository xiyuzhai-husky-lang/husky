use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = new_inner)]
pub struct TypeAssociatedTypeHirDefn {
    pub path: TypeItemPath,
    pub hir_decl: TypeAssociatedTypeHirDecl,
    pub hir_expr_region: HirEagerExprRegion,
}

impl TypeAssociatedTypeHirDefn {
    pub(super) fn new(
        _db: &dyn HirDefnDb,
        _path: TypeItemPath,
        _hir_decl: TypeAssociatedTypeHirDecl,
    ) -> Self {
        todo!()
    }

    pub(super) fn dependencies(self, db: &dyn HirDefnDb) -> HirDefnDependencies {
        ty_associated_ty_hir_defn_dependencies(db, self)
    }

    pub(super) fn version_stamp(self, db: &dyn HirDefnDb) -> HirDefnVersionStamp {
        ty_associated_ty_hir_defn_version_stamp(db, self)
    }
}

#[salsa::tracked(jar = HirDefnJar)]
fn ty_associated_ty_hir_defn_dependencies(
    db: &dyn HirDefnDb,
    hir_defn: TypeAssociatedTypeHirDefn,
) -> HirDefnDependencies {
    todo!()
}

#[salsa::tracked(jar = HirDefnJar)]
fn ty_associated_ty_hir_defn_version_stamp(
    db: &dyn HirDefnDb,
    hir_defn: TypeAssociatedTypeHirDefn,
) -> HirDefnVersionStamp {
    todo!()
}
